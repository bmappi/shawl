#[cfg(all(test, windows))]
speculate::speculate! {
    fn parent() -> String {
        format!("{}/target/debug/shawl.exe", env!("CARGO_MANIFEST_DIR"))
    }

    fn child() -> String {
        format!("{}/target/debug/shawl-child.exe", env!("CARGO_MANIFEST_DIR"))
    }

    fn log_file() -> String {
        format!("{}/target/debug/shawl_for_shawl_rCURRENT.log", env!("CARGO_MANIFEST_DIR"))
    }

    fn log_file_custom_dir() -> String {
        format!("{}/target/debug/log_dir/shawl_for_shawl_rCURRENT.log", env!("CARGO_MANIFEST_DIR"))
    }

    fn log_custom_dir() -> String {
        format!("{}/target/debug/log_dir", env!("CARGO_MANIFEST_DIR"))
    }

    fn delete_log() {
        if log_exists() {
            std::fs::remove_file(log_file()).unwrap();
        }
        if std::path::Path::new(&log_custom_dir()).is_dir() {
            std::fs::remove_dir_all(log_custom_dir()).unwrap();
        }
    }

    fn log_exists() -> bool {
        std::path::Path::new(&log_file()).exists()
    }

    fn run_cmd(args: &[&str]) -> std::process::Output {
        let out = std::process::Command::new(args[0])
                .args(args[1..].iter())
                .output()
                .unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));
        out
    }

    fn run_shawl(args: &[&str]) -> std::process::Output {
        let out = std::process::Command::new(parent())
                .args(args)
                .output()
                .unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));
        out
    }

    before {
        run_cmd(&["sc", "stop", "shawl"]);
        run_cmd(&["sc", "delete", "shawl"]);
        delete_log();
    }

    after {
        run_cmd(&["sc", "stop", "shawl"]);
        run_cmd(&["sc", "delete", "shawl"]);
    }

    describe "shawl add" {
        it "works with minimal arguments" {
            let shawl_output = run_shawl(&["add", "--name", "shawl", "--", &child()]);
            assert_eq!(shawl_output.status.code(), Some(0));

            let sc_output = run_cmd(&["sc", "qc", "shawl"]);
            let pattern = regex::Regex::new(
                r"BINARY_PATH_NAME *: .+shawl\.exe run --name shawl -- .+shawl-child\.exe"
            ).unwrap();
            assert!(pattern.is_match(&String::from_utf8_lossy(&sc_output.stdout)));
        }

        it "handles command parts with spaces" {
            let shawl_output = run_shawl(&["add", "--name", "shawl", "--", "foo bar", "--baz"]);
            assert_eq!(shawl_output.status.code(), Some(0));

            let sc_output = run_cmd(&["sc", "qc", "shawl"]);
            let pattern = regex::Regex::new(
                r#"BINARY_PATH_NAME *: .+shawl\.exe run --name shawl -- "foo bar" --baz"#
            ).unwrap();
            assert!(pattern.is_match(&String::from_utf8_lossy(&sc_output.stdout)));
        }

        it "rejects nonexistent --cwd path" {
            let shawl_output = run_shawl(&["add", "--name", "shawl", "--cwd", "shawl-fake", "--", &child()]);
            assert_eq!(shawl_output.status.code(), Some(2));
        }
    }

    describe "shawl run" {
        it "handles a successful command" {
            run_shawl(&["add", "--name", "shawl", "--", &child()]);
            run_cmd(&["sc", "start", "shawl"]);
            run_cmd(&["sc", "stop", "shawl"]);

            let sc_output = run_cmd(&["sc", "query", "shawl"]);
            let stdout = String::from_utf8_lossy(&sc_output.stdout);

            assert!(stdout.contains("STATE              : 1  STOPPED"));
            assert!(stdout.contains("WIN32_EXIT_CODE    : 0  (0x0)"));
        }

        it "reports a --pass code as success" {
            run_shawl(&["add", "--name", "shawl", "--pass", "1", "--", &child(), "--exit", "1"]);
            run_cmd(&["sc", "start", "shawl"]);
            run_cmd(&["sc", "stop", "shawl"]);

            let sc_output = run_cmd(&["sc", "query", "shawl"]);
            let stdout = String::from_utf8_lossy(&sc_output.stdout);

            assert!(stdout.contains("STATE              : 1  STOPPED"));
            assert!(stdout.contains("WIN32_EXIT_CODE    : 0  (0x0)"));
        }

        it "reports a service-specific error for a failing command" {
            run_shawl(&["add", "--name", "shawl", "--", &child(), "--exit", "7"]);
            run_cmd(&["sc", "start", "shawl"]);
            run_cmd(&["sc", "stop", "shawl"]);

            let sc_output = run_cmd(&["sc", "query", "shawl"]);
            let stdout = String::from_utf8_lossy(&sc_output.stdout);

            assert!(stdout.contains("STATE              : 1  STOPPED"));
            assert!(stdout.contains("WIN32_EXIT_CODE    : 1066  (0x42a)"));
            assert!(stdout.contains("SERVICE_EXIT_CODE  : 7  (0x7)"));
        }

        it "handles a command that times out on stop" {
            run_shawl(&["add", "--name", "shawl", "--", &child(), "--infinite"]);
            run_cmd(&["sc", "start", "shawl"]);
            run_cmd(&["sc", "stop", "shawl"]);
            std::thread::sleep(std::time::Duration::from_secs(4));

            let sc_output = run_cmd(&["sc", "query", "shawl"]);
            let stdout = String::from_utf8_lossy(&sc_output.stdout);
            println!(">>>>>>> {}", stdout);

            assert!(stdout.contains("STATE              : 1  STOPPED"));
            assert!(stdout.contains("WIN32_EXIT_CODE    : 0  (0x0)"));
        }

        it "logs command output by default" {
            run_shawl(&["add", "--name", "shawl", "--", &child()]);
            run_cmd(&["sc", "start", "shawl"]);
            run_cmd(&["sc", "stop", "shawl"]);

            let log = std::fs::read_to_string(log_file()).unwrap();
            assert!(log.contains("stdout: \"shawl-child message on stdout\""));
            assert!(log.contains("stderr: \"shawl-child message on stderr\""));
        }

        it "disables all logging with --no-log" {
            run_shawl(&["add", "--name", "shawl", "--no-log", "--", &child()]);
            run_cmd(&["sc", "start", "shawl"]);
            run_cmd(&["sc", "stop", "shawl"]);

            assert!(!log_exists());
        }

        it "disables command logging with --no-log-cmd" {
            run_shawl(&["add", "--name", "shawl", "--no-log-cmd", "--", &child()]);
            run_cmd(&["sc", "start", "shawl"]);
            run_cmd(&["sc", "stop", "shawl"]);

            let log = std::fs::read_to_string(log_file()).unwrap();
            assert!(!log.contains("shawl-child message on stdout"));
        }

        it "creates log file in custom dir with --log-dir" {
            run_shawl(&["add", "--name", "shawl", "--log-dir", &log_custom_dir(), "--", &child()]);
            run_cmd(&["sc", "start", "shawl"]);
            run_cmd(&["sc", "stop", "shawl"]);

            let log = std::fs::read_to_string(log_file_custom_dir()).unwrap();
            assert!(log.contains("shawl-child message on stdout"));
            assert!(!log_exists()); // Ensure log file hasn't been created next to the .exe
        }

        it "can pass arguments through successfully" {
            run_shawl(&["add", "--name", "shawl", "--pass-start-args", "--", &child()]);
            run_cmd(&["sc", "start", "shawl", "--test"]);
            run_cmd(&["sc", "stop", "shawl"]);

            let log = std::fs::read_to_string(log_file()).unwrap();
            assert!(log.contains("stdout: \"shawl-child test option received\""));
        }

        it "can resolve relative commands with bare executable name and --cwd" {
            std::fs::create_dir(log_custom_dir()).unwrap();
            std::fs::copy(child(), format!("{}/shawl-child-copy.exe", log_custom_dir())).unwrap();

            run_shawl(&["add", "--name", "shawl", "--cwd", &log_custom_dir(), "--", "shawl-child-copy.exe"]);
            run_cmd(&["sc", "start", "shawl"]);
            run_cmd(&["sc", "stop", "shawl"]);

            let log = std::fs::read_to_string(log_file()).unwrap();
            // Example log content, without escaping: "PATH: C:\tmp;\\?\C:\git\shawl\target"
            let pattern = regex::Regex::new(
                &format!(r#"PATH: .+{}"#, &log_custom_dir().replace('/', "\\").replace('\\', "\\\\\\\\"))
            ).unwrap();
            assert!(pattern.is_match(&log));
        }

        it "adds directories to the PATH from --path" {
            std::fs::create_dir(log_custom_dir()).unwrap();
            let extra_path = env!("CARGO_MANIFEST_DIR");

            run_shawl(&["add", "--name", "shawl", "--path", extra_path, "--", &child()]);
            run_cmd(&["sc", "start", "shawl"]);
            run_cmd(&["sc", "stop", "shawl"]);

            let log = std::fs::read_to_string(log_file()).unwrap();
            // Example log content, without escaping: "PATH: C:\tmp;\\?\C:\git\shawl\target"
            let pattern = regex::Regex::new(
                &format!(r#"PATH: .+{}"#, &extra_path.replace('/', "\\").replace('\\', "\\\\\\\\"))
            ).unwrap();
            assert!(pattern.is_match(&log));
        }

        it "loads environment variables from --env" {
            run_shawl(&["add", "--name", "shawl", "--env", "SHAWL_FROM_CLI=custom value", "--", &child()]);
            run_cmd(&["sc", "start", "shawl"]);
            run_cmd(&["sc", "stop", "shawl"]);

            let log = std::fs::read_to_string(log_file()).unwrap();
            let pattern = regex::Regex::new(
                r#"env\.SHAWL_FROM_CLI: Ok\(\\"custom value\\"\)"#
            ).unwrap();
            assert!(pattern.is_match(&log));
        }

        it "waits between restarts with --restart-delay" {
            run_shawl(&["add", "--name", "shawl", "--restart-delay", "180", "--env", "RUST_LOG=shawl=debug", "--", &child(), "--exit", "1"]);
            run_cmd(&["sc", "start", "shawl"]);
            std::thread::sleep(std::time::Duration::from_millis(500));
            run_cmd(&["sc", "stop", "shawl"]);

            let log = std::fs::read_to_string(log_file()).unwrap();
            assert!(log.contains("Delaying 180 ms before restart"));
            assert!(log.lines().filter(|line| line.contains("Sleeping another")).count() > 1);
            assert!(log.contains("Restart delay is complete"));
        }
    }
}
