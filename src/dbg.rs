/*
 * This file is part of rust-gdb.
 *
 * rust-gdb is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * rust-gdb is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with rust-gdb.  If not, see <http://www.gnu.org/licenses/>.
 */

use std::{
    process,
    io::{Write, BufReader, BufWriter, BufRead},
    str
};
use crate::{
    parser,
    msg,
    error::GDBResult
};

pub struct Debugger {
    stdin: BufWriter<process::ChildStdin>,
    stdout: BufReader<process::ChildStdout>,

    #[cfg(feature = "slog")]
    slog: Option<slog::Logger>,
}

impl Debugger {
    fn read_sequence(&mut self) -> GDBResult<Vec<msg::Record>> {
        let mut result = Vec::new();
        let mut line = String::new();
        self.stdout.read_line(&mut line)?;
        while line != "(gdb) \n" && line != "(gdb) \r\n"{
            #[cfg(feature="slog")]
            {
                if let Some(slog) = &self.slog {
                    debug!(slog, "GDB Recv: {}", line.as_str());
                }
            }


            match parser::parse_line(line.as_str()) {
                Ok(resp) => result.push(resp),
                Err(err) => return Err(err),
            }
            line.clear();
            self.stdout.read_line(&mut line)?;
        }
        Ok(result)
    }

    fn read_result_record(&mut self) -> GDBResult<msg::MessageRecord<msg::ResultClass>> {
        loop {
            let sequence = self.read_sequence()?;
            for record in sequence.into_iter() {
                match record {
                    msg::Record::Result(msg) => return Ok(msg),
                    _ => {}
                }
            }
        }
    }

    pub fn send_cmd_raw(&mut self, cmd: &str) -> GDBResult<msg::MessageRecord<msg::ResultClass>> {
        #[cfg(feature="slog")]
        {
            if let Some(slog) = &self.slog {
                debug!(slog, "GDB Send: {}", cmd);
            }
        }


        if cmd.ends_with("\n") {
            write!(self.stdin, "{}", cmd)?;
        } else {
            writeln!(self.stdin, "{}", cmd)?;
        }
        self.stdin.flush()?;
        self.read_result_record()
    }

    #[cfg(feature="slog")]
    pub fn set_logger(&mut self, logger: slog::Logger) {
        self.slog = Some(logger);
    }

    pub fn start() -> GDBResult<Self> {
        let name = ::std::env::var("GDB_BINARY").unwrap_or("gdb".to_string());
        let mut child = process::Command::new(name)
            .args(&["--interpreter=mi"])
            .stdout(process::Stdio::piped())
            .stdin(process::Stdio::piped())
            .stderr(process::Stdio::piped())
            .spawn()?;
        let mut result = Debugger {
            stdin: BufWriter::new(child.stdin.take().expect("broken stdin")),
            stdout: BufReader::new(child.stdout.take().expect("broken stdout")),

            #[cfg(feature="slog")]
            slog: None,
        };
        result.read_sequence()?;
        Ok(result)
    }
}

impl Drop for Debugger {
    fn drop(&mut self) {
        let _ = self.stdin.write_all(b"-gdb-exit\n");
    }
}