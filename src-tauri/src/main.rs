// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

use serde_json::*;
use structs::{MainObject, MainObjects};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
mod structs;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_and_parse_json])
        // .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


// comparator methods

#[tauri::command]
fn read_and_parse_json(json_file_paths: Vec<String>) {

    let file1_path = &json_file_paths[0];
    let file2_path = &json_file_paths[1];
    println!("file1_path: {:?}", file1_path);
    println!("file2_path: {:?}", file2_path);
    let file1 = File::open(file1_path);
    let file2 = File::open(file2_path);

    // getting the result out of the file1 Result<File, Error>
    let file1 = match file1 {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let file2 = match file2 {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let reader1 = BufReader::new(file1);
    let reader2 = BufReader::new(file2);

    println!("{:?}", reader1);
    println!("{:?}", reader2);

    let main_object_1_result: std::prelude::v1::Result<MainObjects, error::Error> = serde_json::from_reader(reader1);
    println!("{:?}", main_object_1_result);

    let main_object_2_result: std::prelude::v1::Result<MainObjects, error::Error> = serde_json::from_reader(reader2);

    // getting the mainobject out of main_object_1_result Result<uknown, Error>

    let main_object_1_result = match main_object_1_result {
        Ok(result) => result,
        Err(error) => panic!("Problem parsing the file: {:?}", error),
    };

    let main_object_2_result = match main_object_2_result {
        Ok(result) => result,
        Err(error) => panic!("Problem parsing the file: {:?}", error),
    };

    // println!("{:?}", main_object_1_result);
    // println!("{:?}", main_object_2_result);

    // find corresponding url in the two files and print all their content
    for mainobject1 in main_object_1_result {
        for mainobject2 in &main_object_2_result {
            compare_request_url(&mainobject1, &mainobject2);
        }
    }
    
}

fn compare_request_url(mainobject1: &MainObject, mainobject2: &MainObject) {
   // compare url and print the difference
    if mainobject1.request.url != mainobject2.request.url {
        println!("Request URL is different");
        println!("File1: {:?}", mainobject1.request.url);
        println!("File2: {:?}", mainobject2.request.url);
    }
    else {
        println!("Request URL is same");
        println!("File1: {:?}", mainobject1.request.url);
        println!("File2: {:?}", mainobject2.request.url);
    }
}