## v1.7.0 (2025-01-16)

* Added: `--restart-delay` option.

## v1.6.0 (2024-11-16)

* Added: `--path-prepend` option.

## v1.5.1 (2024-10-14)

* Fixed: Old log files were not deleted when stored on a Windows network share.

## v1.5.0 (2024-03-02)

* Fixed: Local UNC paths were only simplified for the C drive.
* Added: `shawl --version` to display the program version.
* Changed: Help text is now styled a bit differently.

## v1.4.0 (2023-12-04)

* Added: `--log-rotate` option to control how often the log file rotates.
* Added: `--log-retain` option to control how many old log files are retained.
* Added: `--log-as` option to change the base name of the main log file.
* Added: `--log-cmd-as` option to log the wrapped command's stdout/stderr in a separate file.

## v1.3.0 (2023-10-01)

* Fixed: The path to the Shawl executable was not quoted when it contained spaces.
* Added: `--priority` option to set the process priority.
* Added: `--dependencies` option for `add` command to specify services as dependencies.

## v1.2.1 (2023-08-10)

* Fixed: Possible case in which old log files would not be deleted.
  (Contributed by [Luokun2016](https://github.com/mtkennerly/shawl/pull/33))
* Added: Some guidance in the README related to security.
  (Contributed by [kenvix](https://github.com/mtkennerly/shawl/pull/32))

## v1.2.0 (2023-05-19)

* Fixed: When both `--cwd` and `--path` were specified,
  they would both try to update the command's `PATH` environment variable,
  but the changes from `--cwd` would override the changes from `--path`.
* Changed: When using `--cwd` and `--path`, Shawl now simplifies local UNC paths.
  For example, `\\?\C:\tmp` becomes `C:\tmp`.
  Some programs, notably Command Prompt, don't like UNC paths, so this is intended to broaden compatibility.
* Changed: The CLI output now uses a prettier format, including color.

## v1.1.1 (2022-09-16)

* Fixed `--pass`, `--restart-if`, and `--restart-if-not` not allowing a leading negative number.
* Fixed `--pass`, `--restart-if`, and `--restart-if-not` not requiring a value.
* Fixed `--no-restart`, `--restart-if`, and `--restart-if-not` not being marked as mutually exclusive.
  They had only been marked as exclusive with `--restart`.

## v1.1.0 (2022-01-18)

* Added version to executable properties.
* Added `--log-dir`.
  (Contributed by [oscarbailey-tc](https://github.com/mtkennerly/shawl/pull/19))
* Added `--env`.
* Added `--path`.
* When a custom `--cwd` is set, it is now automatically added to the command's
  PATH to make it easier to write some commands. Specifically, assuming there
  is a `C:\foo\bar\baz.exe`, then `--cwd C:\foo\bar -- baz.exe` will work now,
  but `--cwd C:\foo -- bar\baz.exe` still will not work, because the PATH only
  helps to resolve executable names, not subfolder names.

## v1.0.0 (2021-05-20)

* Shawl now handles computer shutdown/restart, allowing the wrapped program
  to exit gracefully.

## v0.6.2 (2021-03-09)

* Fixed an issue introduced in v0.6.1 where the 32-bit executable was not
  usable on 32-bit systems.
* Changed build process to avoid potential "VCRUNTIME140_1.dll was not found"
  error when using the program.

## v0.6.1 (2020-12-22)

* Updated `windows-service` dependency to avoid a build failure where
  `err-derive` would use a private symbol from `quote`.

## v0.6.0 (2020-03-22)

* Added `--pass-start-args`.
  (Contributed by [Enet4](https://github.com/mtkennerly/shawl/pull/6))
* Added log rotation and service-specific log files.

## v0.5.0 (2020-03-03)

* Added logging of stdout and stderr from commands.
* Added `--no-log` and `--no-log-cmd` options to configure logging.

## v0.4.0 (2019-10-05)

* Added `--cwd` for setting the command's working directory.
* Set default help text width to 80 characters.
* Fixed issue where Shawl would not report an error if it was unable to
  launch the command (e.g., file not found).
* Fixed missing quotes when adding a service if the name or any part of
  the command contained inner spaces.
* Fixed `--pass` and `--stop-timeout` being added to the service command
  configured by `shawl add` even when not explicitly set.

## v0.3.0 (2019-09-30)

* Added `shawl add` for quickly creating a Shawl-wrapped service.
* Moved existing CLI functionality under `shawl run`.
* Generalized `--restart-ok` and `--no-restart-err` into
  `--(no-)restart` and `--restart-if(-not)`.
* Added `--pass` to customize which exit codes are considered successful.

## v0.2.0 (2019-09-22)

* Send ctrl-C to child process first instead of always forcibly killing it.
* Report command failure as a service-specific error to Windows.
* Added `--stop-timeout` option.

## v0.1.0 (2019-09-22)

* Initial release.
