// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate maskerad_object_pool;
extern crate maskerad_memory_allocators;
extern crate maskerad_filesystem;
extern crate maskerad_data_parser;
extern crate maskerad_gameobject_model;
extern crate gltf;
extern crate lewton;
extern crate imagefmt;

pub mod resources;
pub mod properties;
pub mod resource_manager;
pub mod property_manager;

/*
    functionalities :
    - ensure that only one resource is loaded in memory, at any given time. -> hashmap guid ?
    - manage the lifetime of each resource.                     -> hashmap guid ?               OK
    - load needed resources, unload unneeded resources.                                         OK
    - handle composite structures.
    - maintain referential integrity (internal and external).
    - manage memory usage and how the data is placed in memory. -> custom allocators
    - custom-processing, on a per-resource-type basis.
    - a unified interface, through the form of a memory manager. -> one structure               OK
    - handle streaming, if the fs support it.

    resource file organization : a simple one, to begin with :
    - no zip or pak files containing a lot of data like love2D and Unreal.
    - just a file.


    file formats :
    - textures : tga.
    - 3D models : glTF.
    - Audio : ogg.
    - or maybe make an offline tool to "compile" data in bin form.

    Resources must have an ID : the filesystem path, or a hash generally.

    How to guarantee only one resource loaded at a time : a hashmap !
    Hashmap<ID, value> -> ID probably filepath (PathBuf or Path ?), value probably a pointer (since we'll use our allocators).

    WE NEED :
    - A memory manager structure, with a registry (hashmap)
    - our allocators
*/

/*
    A game is made of resources : meshes, materials, texture, sharder, audioclip, geometric primitives...
_____________________________________________________________________________________________________
    Those resources must be managed by :
    - Offline tools (to create them) = Blender, Krita, Ardour, Audacity, Inkscape, GIMP...
    - in-engine tools (load, unload, find, modify...) = the resource manager itself.
_____________________________________________________________________________________________________
    General architecture according to some books and infos -> two parts :
    - Part one :    A component to manage the offline tools. Transform the created resources, by offline tools,
                    to a format manageable by the engine (most data format contained way too much data for usage in a game engine).

    - Part two :    A component to manage resources at runtime -> loading and unloading of resources when needed.
_____________________________________________________________________________________________________
    PART ONE

    Offline resource management :
    - version control system to store assets -> git lfs, hacky symlink solution

    Resource DB :
    - Assets need to pass a conditioning pipeline to be usable by a game engine.
    - need to convert the asset to a binary format usable by GA.

    Conditioning pipeline :
    - what compression algorithm is the most efficient for this type of bitmap ?
    - The frames to keep from a blender animation ?

    => We need metadata, to know how to process an asset.

    Form of a resource DB :
    - config files (xml, toml...) "linked" to assets, which encode the resource building metadata.
    - A real goddamn DB, like MariaDB, PostGreSQL. ->   Looks like PostGreSQL support big binary objects
                                                        and a rust driver exists (https://github.com/sfackler/rust-postgres)

   Resource DB functionality :
   - deal with multiple type of resources, in a relatively consistent manner.
   - create new resources
   - delete resources
   - inspect, modify resource
   - "physically" move resource file from one place to another
   - a resource can reference other resources (mesh has a material...)
   - guarantee referential integrity of resources (if a mesh reference a material, and this one is deleted/moved, warn the user about it and remove/update the reference from the mesh)
   - revision history (who did what when)
   - search query

   some references :
   -    Unreal engine browser (UnrealEd -> THE tool. the editor IS PART of the engine, not a separate software using the engine)
        The resources are created IN the engine and viewed IN the engine.
        They have a generic resource browser, every resources can be inspected/added/modified/removed from this tool.
        Every resources must be IMPORTED into the DB, which allows Unreal to perform a validity check.

        drawbacks of unreal approach :
        - resources stored in big binary files -> annoying for source version control.
        - only one package for hundreds of resources -> only one guy can modify at any time.


   -    Naughty dog solution : at first, a MySQL DB with a custom user interface as a front-end.
        Problems : revision history non-existent, roll-back difficult. Hard to administer efficiently.

        New solution, XML config files stored in source version control + command line programs to build actors and levels.


  ASSET CONDITIONING PIPELINE

  It's a pipeline, natives assets (.obj, .psd...) are passed to various tools as input. Assets consumable by the engine
  is the output.

  Tools :
  - Exporter (generally, a DCC plugin to transform from a native format to a manipulable format)
    if the native format is an open standard, or relatively easy to read and parse, we can go to
    the second phase

  - Resource compiler ("prepare" raw data to make them game-ready). For example, compress a bitmap,
    re-arrange mesh triangles...

  - Resource linker (some resources need to be loaded in a single package to be game -ready). For example,
    a mesh has materials, bitmaps, skeletons...

    It just works like compiled languages -> compile sources files to intermediate objects, link them
    together... boom, an executable.

    And just like compilation and linkage in a compiled language, there's a build dependency order
    (build the skeleton before building the animations).
    Directed acyclic graph and stuff like that ? It's interdependency, just like a package manager right ?


    FIRST DRAFT OF OUR OFFLINE RESOURCE MANAGER:
    - The naughty dog solution looks cool. maybe choose another config file.
_____________________________________________________________________________________________________

    PART TWO

    Runtime resource managment.

    functionality :
    - ONLY ONE COPY of a resource in memory at any time.
    - manage the lifetime of the resource. When no more used, drop it. Sounds like reference counting.
    - load needed resources, unload unneeded resources.
    - load composite resources -> resources referencing other resources.
    - maintain referential integrity -> composite resources.
    - manage memory usage.
    - permit custom processing on a resources after it has been loaded.
    - handle resource streaming.
    - have an interface to manage resources.

    Directory organization (the resource manager doesn't care, just for developer convenience):
    Root
        Resources
            NPC
                NPC1
                    blabla...
                NPC2
                    blabla...
            Player
                blabla
            Weapons
                blabla
            Levels
                blabla
            Objects
                blabla
            Consumables
                blabla

   file formats like ZIP can be beneficial to regroup files in one compressable package:
   - reduced seek time
   - reduced file open time
   Unreal does something like that with .pak files.

   We may need custom file formats, if the open standards one (JPEG, COLLADA, OBJ...)
   doesn't give us all the data we need.

   resources need GUIDs

   Registry of loaded resources -> ensure data loaded one time only, at any given time :

   simple solution: hashmap.
*/