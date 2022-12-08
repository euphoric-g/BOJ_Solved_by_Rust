
std::process (https://doc.rust-lang.org/std/fs/struct.File.html)
    Structs
        Child   -> Representation of a running or exited child process.
        ChildStderr -> A handle to a child process's stderr
        ChildStdin -> A handle to a child process's stdin
        ChildStdout -> A handle to a child process's stdout
        Command -> A process builder, providing find-grained control over how a new process should be spawned.
        {
            Methods
                arg -> Adds an argument to pass to the program.
                    Command::new("ls").arg("-l").arg("-a").spawn().expect("ls command failed to start");
                args -> Adds multiple arguments to pass to the program.
                    Command::new("ls").args(["-l", "-a"]).spawn().expect("ls command failed to start");
                current_dir
                env -> Inserts or updates an environment variable mapping.
                    Command::new("ls").env("PATH", "/bin").spawn().expect("ls command failed to start");
                env_clear
                env_remove
                envs
                    let filtered_env : HashMap<String, String> = env::vars().filter(&(ref k, _)|
                        k == "TERM" || k == "TZ" || k == "LANG" || k == "PATH"
                    ).collect();
                    Command::new("printenv").stdin(Stdio::null()).stdout(Stdio::inherit()).env_clear()
                        .envs(&filtered_env).spawn().expect("printenv failed to start");
                get_args
                get_current_dir -> Sets the working directory for the child process.
                get_envs
                get_program -> Returns the path to the program that was given to Command::new.
                    let cmd = Command::new("echo");
                    assert_eq!(cmd.get_program(), "echo");
                new -> Constructs a new Command for launching the program at path program(:String) with the following default configuration : No arguments, Inherit the current process's environment, working directory, Inherit stdin/stdout/stderr for spawn or status, but create pipes for output.
                    Command::new("sh").spawn().expect("sh command failed to start");
                output -> Executes the command as a child process, waiting for it to finish and collecting all of its output. (Returns Result<Output>)
                spawn -> Executes the command as a child process, returning a handle to it. By default, stdin, stdout, stderr are inherited from the parent.
                status
                stderr
                stdin -> Configuration for the child process's standard input handle
                stdout -> Configuration for the child process's standard output handle
        }
        CommandArgs -> An iterator over the command arguments.
        CommandEnvs -> An iterator over the environment variables.
        ExitCode -> This type represents the status code the current process can return to its parent under normal termination.
        ExitStatus -> Describes the result of a process after it has terminated.
        Output -> The output of a finished process.
        Stdio -> Describes what to do with a standard I/O stream for a child process when passed to the stdin, stdout, stderr methods of Command