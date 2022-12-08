# BOJ_Solved_by_Rust

std::fs (https://doc.rust-lang.org/std/fs/index.html)
    Structs
        FileTimes -> Representation of the various timestamps on a file (experimental feature)
        DirBuilder -> {
            let builder = DirBuilder::new(); // Creates a new set of options with default mode/security settings for all platforms and also non-recursive.
            builder.recursive(true); // Indicates that directories should be created recursively.
            builder.create("/tmp/foo/bar/baz").unwrap(); // Creates the specified directory with the options configured in this builder. It is considered an error if the directory already exists unless recursive mode is enabled.
        }
        DirEntry -> Entries returned by the ReadDir iterator
        {
            Methods
                file_name
                file_type
                metadata
                path

        }
        File -> {
            Methods
                create -> Opens a file in write-only mode. This function will create a file if it doesn't exist, and will truncate it if it does exist.
                metadata
                open -> Attempts to open a file in read-only mode.
                    let mut f = File::open("foo.txt")?;
                options -> Returns a new OpenOptions object.
                    let mut f = File::options().append(true).read(true).open("example.log")?;
                set_len
                set_modified
                set_permissions
                set_times
                sync_all -> Attempts to sync all OS-internal metadata to disk.
                    f.sync_all()?;
                sync_data -> Attempts to sync file content to disk.
                try_clone -> Creates a new File instance that shares the same underlying file handle.
                    let mut file = File::open("foo.txt")?;
                    let file_copy = file.try_clone()?;
            Trait Implementations (read)
                read -> Pull some bytes from this source into the specified buffer, returning how many bytes were read.
                read_to_string -> Red all bytes until EOF in this source, appending them to buf. (buf: &mut String)
                bytes -> Transforms this Read instance to an Iterator over its bytes
                read_exact -> Read the exact number of bytes required to fill buf. (buf: &mut [u8])
            Trait Implementations (write)
                write -> Write a buffer into this writer, returning how many bytes were written. (buf: &[u8])
                flush -> Flush this output stream, ensuring that all intermediately buffered contents reach their destination.
                write_fmt -> Writes a formatted string into this writer, returning any error encountered.
            

        }
        FileType -> {
            Methods
                is_dir
                is_file
                is_symlink
        }
        Metadata -> {
            Methods
                file_type -> Returns the file type for this metadata. (std::fs::FileType)
                is_dir
                is_file
                is_symlink
                len -> Returns the size of the file, in bytes, this metadata is for.
                permissions -> Returns the permissions of the file this metadata is for.
        }
        OpenOptions -> {
            Methods
                append -> append(true) == write(true).append(true)
                create
                create_new
                new
                open
                read
                truncate
                write
        }
        Permissions -> {
            Methods
                readonly -> Returns true if these permissions describe a readonly file.
                set_readonly -> This operation does not modify the filesystem. Use the set_permissions function instead.
        }
        ReadDir -> Iterator over the entries in a directory
    Functions
        try_exists  -> Return Ok(true) if the path points at an existing entity. (Experimental feature)
        canonicalize
        copy
        create_dir
        create_dir_all  -> Recursively create a directory and all of its parent components if they are missing.
        hard_link
        metadata
        read    -> Read the entire contents of a file into a bytes vector.
        read_dir    -> Returns an iterator over the entries within a directory.
        read_link
        read_to_string -> Read the entire contents of a file into a string.
        remove_dir
        remove_dir_all
        remove_file
        rename
        set_permissions -> Changes the permissions found on a file or a directory
        soft_link (deprecated)
        symlink_metadata
        write -> Write a slice as the entire contents of a file.