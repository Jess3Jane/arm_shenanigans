use std::process::Command;
use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;
use tempdir::TempDir;

#[derive(Debug)]
pub enum AssemblerError {
    AssembleError(std::process::Output),
    CopyError(std::process::Output),
    IOError(std::io::Error),
}

impl Display for AssemblerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            AssemblerError::AssembleError(e) => {
                format!("\x1b[31;1mError assembling code\x1b[39m (process returned {:?})\nstdout:\n{}\x1b[0m\n\n\x1b[1mstderr:\x1b[0m\n{}",
                    match e.status.code(){ 
                        Some(v) => format!("{}", v), 
                        None => String::from("terminated by signal")
                    },
                    match String::from_utf8(e.stdout.clone()){
                        Ok(s) => s, Err(_) => format!("{:?}", e.stdout)},
                    match String::from_utf8(e.stderr.clone()){
                        Ok(s) => s, Err(_) => format!("{:?}", e.stderr)},
                    )
            },
            AssemblerError::CopyError(e) => {
                format!("\x1b[31;1mError copying object\x1b[39m (process returned {:?})\nstdout:\n{}\x1b[0m\n\n\x1b[1mstderr:\x1b[0m\n{}",
                    match e.status.code(){ 
                        Some(v) => format!("{}", v), 
                        None => String::from("terminated by signal")
                    },
                    match String::from_utf8(e.stdout.clone()){
                        Ok(s) => s, Err(_) => format!("{:?}", e.stdout)},
                    match String::from_utf8(e.stderr.clone()){
                        Ok(s) => s, Err(_) => format!("{:?}", e.stderr)},
                    )
            },
            AssemblerError::IOError(e) => {
                return e.fmt(f);
            }
        })
    }
}

impl Error for AssemblerError {
    fn cause(&self) -> Option<&Error> {
        match self {
            AssemblerError::AssembleError(e) => { None },
            AssemblerError::CopyError(e) => { None },
            AssemblerError::IOError(e) => { e.cause() }
        }
    }
}

pub fn assemble(code: &str) -> Result<Vec<u8>, AssemblerError> {
    let tmp_dir = match TempDir::new("arm_assembler") {
        Ok(td) => td,
        Err(e) => {return Err(AssemblerError::IOError(e));},
    };
    { match match File::create(tmp_dir.path().join("code.s")) {
        Ok(f) => f,
        Err(e) => {return Err(AssemblerError::IOError(e));},
        }.write_all(code.as_bytes()) {
            Ok(_) => {},
            Err(e) => {return Err(AssemblerError::IOError(e));},
        }};

    let ret = match Command::new("arm-none-eabi-as")
        .current_dir(tmp_dir.path())
        .arg("code.s")
        .arg("-o").arg("object.o")
        .output() {
            Ok(v) => v,
            Err(e) => {return Err(AssemblerError::IOError(e));},
        };
    if !ret.status.success() {
        return Err(AssemblerError::AssembleError(ret));
    }

    let ret = match Command::new("arm-none-eabi-objcopy")
        .current_dir(tmp_dir.path())
        .arg("-O").arg("binary")
        .arg("object.o")
        .arg("binfile")
        .output() {
            Ok(v) => v,
            Err(e) => {return Err(AssemblerError::IOError(e));},
        };
    if !ret.status.success() {
        return Err(AssemblerError::CopyError(ret));
    }

    let mut data = Vec::new();
    match match File::open(tmp_dir.path().join("binfile")) {
        Ok(f) => f,
        Err(e) => {return Err(AssemblerError::IOError(e));},
    }.read_to_end(&mut data) {
        Ok(_) => {},
        Err(e) => {return Err(AssemblerError::IOError(e));},
    };

    Ok(data)
}
