//! # Process and Pipes
//! # 进程与管道
//! 
//! In this exercise, you will learn how to create child processes and communicate through pipes.
//! 在本练习中，你将学习如何创建子进程并通过管道进行通信。
//! 
//! ## Concepts
//! ## 概念
//! - `std::process::Command` creates child processes (corresponds to `fork()` + `execve()` system calls)
//! - `std::process::Command` 创建子进程（对应 `fork()` + `execve()` 系统调用）
//! - `Stdio::piped()` sets up pipes (corresponds to `pipe()` + `dup2()` system calls)
//! - `Stdio::piped()` 设置管道（对应 `pipe()` + `dup2()` 系统调用）
//! - Communicate with child processes via stdin/stdout
//! - 通过 stdin/stdout 与子进程通信
//! - Obtain child process exit status (corresponds to `waitpid()` system call)
//! - 获取子进程退出状态（对应 `waitpid()` 系统调用）
//! 
//! ## OS Concepts Mapping
//! ## 操作系统概念映射
//! This exercise demonstrates user‑space abstractions over underlying OS primitives:
//! 本练习展示了用户空间对底层操作系统原语的抽象：
//! - **Process creation**: Rust's `Command::new()` internally invokes `fork()` to create a child process,
//!   then `execve()` (or equivalent) to replace the child's memory image with the target program.
//! - **进程创建**：Rust 的 `Command::new()` 内部调用 `fork()` 创建子进程，
//!   然后调用 `execve()`（或等效函数）用目标程序替换子进程的内存映像。
//! - **Inter‑process communication (IPC)**: Pipes are kernel‑managed buffers that allow one‑way data
//!   flow between related processes. The `pipe()` system call creates a pipe, returning two file
//!   descriptors (read end, write end). `dup2()` duplicates a file descriptor, enabling redirection
//!   of standard input/output.
//! - **进程间通信 (IPC)**：管道是由内核管理的缓冲区，允许相关进程之间的单向数据流。
//!   `pipe()` 系统调用创建一个管道，返回两个文件描述符（读取端、写入端）。
//!   `dup2()` 复制文件描述符，实现标准输入/输出的重定向。
//! - **Resource management**: File descriptors (including pipe ends) are automatically closed when
//!   their Rust `Stdio` objects are dropped, preventing resource leaks.
//! - **资源管理**：当 Rust 的 `Stdio` 对象被丢弃时，文件描述符（包括管道端）会自动关闭，
//!   防止资源泄漏。
//! 
//! ## Exercise Structure
//! ## 练习结构
//! 1. **Basic command execution** (`run_command`) – launch a child process and capture its stdout.
//! 1. **基本命令执行** (`run_command`) – 启动子进程并捕获其标准输出。
//! 2. **Bidirectional pipe communication** (`pipe_through_cat`) – send data to a child process (`cat`)
//!    and read its output.
//! 2. **双向管道通信** (`pipe_through_cat`) – 向子进程 (`cat`) 发送数据并读取其输出。
//! 3. **Exit code retrieval** (`get_exit_code`) – obtain the termination status of a child process.
//! 3. **退出代码获取** (`get_exit_code`) – 获取子进程的终止状态。
//! 4. **Advanced: error‑handling version** (`run_command_with_result`) – learn proper error propagation.
//! 4. **高级：错误处理版本** (`run_command_with_result`) – 学习正确的错误传播。
//! 5. **Advanced: complex bidirectional communication** (`pipe_through_grep`) – interact with a filter
//!    program that reads multiple lines and produces filtered output.
//! 5. **高级：复杂双向通信** (`pipe_through_grep`) – 与读取多行并产生过滤输出的过滤程序交互。
//! 
//! Each function includes a `TODO` comment indicating where you need to write code.
//! 每个函数都包含一个 `TODO` 注释，指示你需要编写代码的位置。
//! Run `cargo test` to check your implementations.
//! 运行 `cargo test` 检查你的实现。

use std::io::{self, Read, Write};
use std::process::{Command, Stdio};

/// Execute the given shell command and return its stdout output.
/// 执行给定的 shell 命令并返回其标准输出。
///
/// For example: `run_command("echo", &["hello"])` should return `"hello\n"`
/// 例如：`run_command("echo", &["hello"])` 应该返回 `"hello\n"`
///
/// # Underlying System Calls
/// # 底层系统调用
/// - `Command::new(program)` → `fork()` + `execve()` family
/// - `Command::new(program)` → `fork()` + `execve()` 家族
/// - `Stdio::piped()` → `pipe()` + `dup2()` (sets up a pipe for stdout)
/// - `Stdio::piped()` → `pipe()` + `dup2()`（为标准输出设置管道）
/// - `.output()` → `waitpid()` (waits for child process termination)
/// - `.output()` → `waitpid()`（等待子进程终止）
///
/// # Implementation Steps
/// # 实现步骤
/// 1. Create a `Command` with the given program and arguments.
/// 1. 使用给定的程序和参数创建一个 `Command`。
/// 2. Set `.stdout(Stdio::piped())` to capture the child's stdout.
/// 2. 设置 `.stdout(Stdio::piped())` 以捕获子进程的标准输出。
/// 3. Call `.output()` to execute the child and obtain its `Output`.
/// 3. 调用 `.output()` 执行子进程并获取其 `Output`。
/// 4. Convert the `stdout` field (a `Vec<u8>`) into a `String`.
/// 4. 将 `stdout` 字段（一个 `Vec<u8>`）转换为 `String`。
pub fn run_command(program: &str, args: &[&str]) -> String {
    // TODO: Use Command::new to create process
    // TODO: 使用 Command::new 创建进程
    // TODO: Set stdout to Stdio::piped()
    // TODO: 设置 stdout 为 Stdio::piped()
    // TODO: Execute with .output() and get output
    // TODO: 使用 .output() 执行并获取输出
    // TODO: Convert stdout to String and return
    // TODO: 将 stdout 转换为 String 并返回
    //todo!()
    let output = Command::new(program)
        .args(args)
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.to_string()
        
}

/// Write data to child process (cat) stdin via pipe and read its stdout output.
/// 通过管道向子进程 (cat) 的标准输入写入数据并读取其标准输出。
///
/// This demonstrates bidirectional pipe communication between parent and child processes.
/// 这演示了父进程和子进程之间的双向管道通信。
///
/// # Underlying System Calls
/// # 底层系统调用
/// - `Command::new("cat")` → `fork()` + `execve("cat")`
/// - `Command::new("cat")` → `fork()` + `execve("cat")`
/// - `Stdio::piped()` (twice) → `pipe()` creates two pipes (stdin & stdout) + `dup2()` redirects them
/// - `Stdio::piped()`（两次）→ `pipe()` 创建两个管道（标准输入和标准输出）+ `dup2()` 重定向它们
/// - `ChildStdin::write_all()` → `write()` to the pipe's write end
/// - `ChildStdin::write_all()` → `write()` 到管道的写入端
/// - `drop(stdin)` → `close()` on the write end, sending EOF to child
/// - `drop(stdin)` → `close()` 写入端，向子进程发送 EOF
/// - `ChildStdout::read_to_string()` → `read()` from the pipe's read end
/// - `ChildStdout::read_to_string()` → `read()` 从管道的读取端
///
/// # Ownership and Resource Management
/// # 所有权和资源管理
/// Rust's ownership system ensures pipes are closed at the right time:
/// Rust 的所有权系统确保管道在正确的时间关闭：
/// 1. The `ChildStdin` handle is owned by the parent; writing to it transfers data to the child.
/// 1. `ChildStdin` 句柄由父进程拥有；向其写入数据会将数据传输给子进程。
/// 2. After writing, we explicitly `drop(stdin)` (or let it go out of scope) to close the write end.
/// 2. 写入后，我们显式 `drop(stdin)`（或让其超出作用域）以关闭写入端。
/// 3. Closing the write end signals EOF to `cat`, causing it to exit after processing all input.
/// 3. 关闭写入端向 `cat` 发送 EOF，使其在处理完所有输入后退出。
/// 4. The `ChildStdout` handle is then read to completion; dropping it closes the read end.
/// 4. 然后读取 `ChildStdout` 句柄直到完成；丢弃它会关闭读取端。
///
/// Without dropping `stdin`, the child would wait forever for more input (pipe never closes).
/// 如果不丢弃 `stdin`，子进程将永远等待更多输入（管道永远不会关闭）。
///
/// # Implementation Steps
/// # 实现步骤
/// 1. Create a `Command` for `"cat"` with `.stdin(Stdio::piped())` and `.stdout(Stdio::piped())`.
/// 1. 为 `"cat"` 创建一个 `Command`，设置 `.stdin(Stdio::piped())` 和 `.stdout(Stdio::piped())`。
/// 2. `.spawn()` the command to obtain a `Child` with `stdin` and `stdout` handles.
/// 2. `.spawn()` 命令以获取带有 `stdin` 和 `stdout` 句柄的 `Child`。
/// 3. Write `input` bytes to the child's stdin (`child.stdin.take().unwrap().write_all(...)`).
/// 3. 向子进程的标准输入写入 `input` 字节 (`child.stdin.take().unwrap().write_all(...)`)。
/// 4. Drop the stdin handle (explicit `drop` or let it go out of scope) to close the pipe.
/// 4. 丢弃标准输入句柄（显式 `drop` 或让其超出作用域）以关闭管道。
/// 5. Read the child's stdout (`child.stdout.take().unwrap().read_to_string(...)`).
/// 5. 读取子进程的标准输出 (`child.stdout.take().unwrap().read_to_string(...)`)。
/// 6. Wait for the child to exit with `.wait()` (or rely on drop‑wait).
/// 6. 使用 `.wait()` 等待子进程退出（或依赖于 drop‑wait）。
pub fn pipe_through_cat(input: &str) -> String {
    // TODO: Create "cat" command, set stdin and stdout to piped
    // TODO: 创建 "cat" 命令，设置 stdin 和 stdout 为管道
    // TODO: Spawn process
    // TODO: 启动进程
    // TODO: Write input to child process stdin
    // TODO: 向子进程的 stdin 写入输入
    // TODO: Drop stdin to close pipe (otherwise cat won't exit)
    // TODO: 丢弃 stdin 以关闭管道（否则 cat 不会退出）
    // TODO: Read output from child process stdout
    // TODO: 从子进程的 stdout 读取输出
    todo!()
}

/// Get child process exit code.
/// 获取子进程退出代码。
/// Execute command `sh -c {command}` and return the exit code.
/// 执行命令 `sh -c {command}` 并返回退出代码。
///
/// # Underlying System Calls
/// # 底层系统调用
/// - `Command::new("sh")` → `fork()` + `execve("/bin/sh")`
/// - `Command::new("sh")` → `fork()` + `execve("/bin/sh")`
/// - `.args(["-c", command])` passes the shell command line
/// - `.args(["-c", command])` 传递 shell 命令行
/// - `.status()` → `waitpid()` (waits for child and retrieves exit status)
/// - `.status()` → `waitpid()`（等待子进程并检索退出状态）
/// - `ExitStatus::code()` extracts the low‑byte exit code (0‑255)
/// - `ExitStatus::code()` 提取低字节退出代码 (0‑255)
///
/// # Implementation Steps
/// # 实现步骤
/// 1. Create a `Command` for `"sh"` with arguments `["-c", command]`.
/// 1. 为 `"sh"` 创建一个 `Command`，参数为 `["-c", command]`。
/// 2. Call `.status()` to execute the shell and obtain an `ExitStatus`.
/// 2. 调用 `.status()` 执行 shell 并获取 `ExitStatus`。
/// 3. Use `.code()` to get the exit code as `Option<i32>`.
/// 3. 使用 `.code()` 获取退出代码作为 `Option<i32>`。
/// 4. If the child terminated normally, return the exit code; otherwise return a default.
/// 4. 如果子进程正常终止，返回退出代码；否则返回默认值。
pub fn get_exit_code(command: &str) -> i32 {
    // TODO: Use Command::new("sh").args(["-c", command])
    // TODO: 使用 Command::new("sh").args(["-c", command])
    // TODO: Execute and get status
    // TODO: 执行并获取状态
    // TODO: Return exit code
    // TODO: 返回退出代码
    todo!()
}

/// Execute the given shell command and return its stdout output as a `Result`.
/// 执行给定的 shell 命令并以 `Result` 形式返回其标准输出。
///
/// This version properly propagates errors that may occur during process creation,
/// execution, or I/O (e.g., command not found, permission denied, broken pipe).
/// 此版本正确传播在进程创建、执行或 I/O 期间可能发生的错误（例如，命令未找到、权限被拒绝、管道损坏）。
///
/// # Underlying System Calls
/// # 底层系统调用
/// Same as `run_command`, but errors are captured from the OS and returned as `Err`.
/// 与 `run_command` 相同，但错误从操作系统捕获并作为 `Err` 返回。
///
/// # Error Handling
/// # 错误处理
/// - `Command::new()` only constructs the builder; errors occur at `.output()`.
/// - `Command::new()` 仅构造构建器；错误发生在 `.output()`。
/// - `.output()` returns `Result<Output, std::io::Error>`.
/// - `.output()` 返回 `Result<Output, std::io::Error>`。
/// - `String::from_utf8()` may fail if the child's output is not valid UTF‑8.
///   In that case we return an `io::Error` with kind `InvalidData`.
/// - 如果子进程的输出不是有效的 UTF‑8，`String::from_utf8()` 可能会失败。
///   在这种情况下，我们返回一个类型为 `InvalidData` 的 `io::Error`。
///
/// # Implementation Steps
/// # 实现步骤
/// 1. Create a `Command` with the given program and arguments.
/// 1. 使用给定的程序和参数创建一个 `Command`。
/// 2. Set `.stdout(Stdio::piped())`.
/// 2. 设置 `.stdout(Stdio::piped())`。
/// 3. Call `.output()` and propagate any `io::Error`.
/// 3. 调用 `.output()` 并传播任何 `io::Error`。
/// 4. Convert `stdout` to `String` with `String::from_utf8`; if that fails, map to an `io::Error`.
/// 4. 使用 `String::from_utf8` 将 `stdout` 转换为 `String`；如果失败，映射为 `io::Error`。
pub fn run_command_with_result(program: &str, args: &[&str]) -> io::Result<String> {
    // TODO: Create "grep" command with pattern, set stdin and stdout to piped
    // TODO: 创建 "grep" 命令并指定 pattern，设置 stdin 和 stdout 为管道
    // TODO: Spawn process
    // TODO: 启动进程
    // TODO: Write input lines to child stdin
    // TODO: 向子进程的 stdin 写入输入行
    // TODO: Drop stdin to close pipe
    // TODO: 丢弃 stdin 以关闭管道
    // TODO: Read output from child stdout line by line
    // TODO: 逐行从子进程的 stdout 读取输出
    // TODO: Collect and return matching lines
    // TODO: 收集并返回匹配的行
    todo!()
}

/// Interact with `grep` via bidirectional pipes, filtering lines that contain a pattern.
/// 通过双向管道与 `grep` 交互，过滤包含模式的行。
///
/// This demonstrates complex parent‑child communication: the parent sends multiple
/// lines of input, the child (`grep`) filters them according to a pattern, and the
/// parent reads back only the matching lines.
/// 这演示了复杂的父子进程通信：父进程发送多行输入，子进程 (`grep`) 根据模式过滤它们，
/// 父进程只读取匹配的行。
///
/// # Underlying System Calls
/// # 底层系统调用
/// - `Command::new("grep")` → `fork()` + `execve("grep")`
/// - `Command::new("grep")` → `fork()` + `execve("grep")`
/// - Two pipes (stdin & stdout) as in `pipe_through_cat`
/// - 两个管道（标准输入和标准输出），如 `pipe_through_cat` 中所示
/// - Line‑by‑line writing and reading to simulate interactive filtering
/// - 逐行写入和读取以模拟交互式过滤
///
/// # Implementation Steps
/// # 实现步骤
/// 1. Create a `Command` for `"grep"` with argument `pattern`, and both ends piped.
/// 1. 为 `"grep"` 创建一个 `Command`，参数为 `pattern`，并将两端设置为管道。
/// 2. `.spawn()` the command, obtaining `Child` with `stdin` and `stdout` handles.
/// 2. `.spawn()` 命令，获取带有 `stdin` 和 `stdout` 句柄的 `Child`。
/// 3. Write each line of `input` (separated by `'\n'`) to the child's stdin.
/// 3. 将 `input` 的每一行（以 `'\n'` 分隔）写入子进程的标准输入。
/// 4. Close the write end (drop stdin) to signal EOF.
/// 4. 关闭写入端（丢弃 stdin）以发送 EOF 信号。
/// 5. Read the child's stdout line by line, collecting matching lines.
/// 5. 逐行读取子进程的标准输出，收集匹配的行。
/// 6. Wait for the child to exit (optional; `grep` exits after EOF).
/// 6. 等待子进程退出（可选；`grep` 在 EOF 后退出）。
/// 7. Return the concatenated matching lines as a single `String`.
/// 7. 将连接的匹配行作为单个 `String` 返回。
///
pub fn pipe_through_grep(pattern: &str, input: &str) -> String {
    // TODO: Create "grep" command with pattern, set stdin and stdout to piped
    // TODO: Spawn process
    // TODO: Write input lines to child stdin
    // TODO: Drop stdin to close pipe
    // TODO: Read output from child stdout line by line
    // TODO: Collect and return matching lines
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_echo() {
        let output = run_command("echo", &["hello"]);
        assert_eq!(output.trim(), "hello");
    }

    #[test]
    fn test_run_with_args() {
        let output = run_command("echo", &["-n", "no newline"]);
        assert_eq!(output, "no newline");
    }

    #[test]
    fn test_pipe_cat() {
        let output = pipe_through_cat("hello pipe!");
        assert_eq!(output, "hello pipe!");
    }

    #[test]
    fn test_pipe_multiline() {
        let input = "line1\nline2\nline3";
        assert_eq!(pipe_through_cat(input), input);
    }

    #[test]
    fn test_exit_code_success() {
        assert_eq!(get_exit_code("true"), 0);
    }

    #[test]
    fn test_exit_code_failure() {
        assert_eq!(get_exit_code("false"), 1);
    }

    #[test]
    fn test_run_command_with_result_success() {
        let result = run_command_with_result("echo", &["hello"]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().trim(), "hello");
    }

    #[test]
    fn test_run_command_with_result_nonexistent() {
        let result = run_command_with_result("nonexistent_command_xyz", &[]);
        // Should be an error because command not found
        assert!(result.is_err());
    }

    #[test]
    fn test_pipe_through_grep_basic() {
        let input = "apple\nbanana\ncherry\n";
        let output = pipe_through_grep("a", input);
        // grep outputs matching lines with newline
        assert_eq!(output, "apple\nbanana\n");
    }

    #[test]
    fn test_pipe_through_grep_no_match() {
        let input = "apple\nbanana\ncherry\n";
        let output = pipe_through_grep("z", input);
        // No lines match -> empty string
        assert_eq!(output, "");
    }

    #[test]
    fn test_pipe_through_grep_multiline() {
        let input = "first line\nsecond line\nthird line\n";
        let output = pipe_through_grep("second", input);
        assert_eq!(output, "second line\n");
    }
}
