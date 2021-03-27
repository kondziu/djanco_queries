use djanco::*;
use djanco::log::*;
use clap::{Arg, App};

mod queries;

use std;

fn main() {

    // database path to make queries to.
    let mut dataset_path = "/Users/stefaniemuroyalei/Documents/prague/work/codedj-parasite/example-dataset";
    let mut cache_path = "/Users/stefaniemuroyalei/Documents/prague/work/codedj-parasite/cache";

    // vector of queries, every query is added to this vector.
    let arr_queries: Vec< fn (a: djanco::data::Database, b: &str)> = 
                vec![
                    queries::query1, queries::query2, queries::query3,
                    queries::query4, queries::query5, queries::query6,
                    queries::query7, queries::query8
                ];

    // Define command line flags
    let matches = App::new("Replication of Queries for Various Papers Using Djanco")
        .author("Stefanie Muroya")
        .arg(Arg::with_name("dataset")
            .short("d")
            .long("dataset")
            .takes_value(true)
            .help("The path of the dataset that contains repos. If blank, then de compiled value of the constant dataset_path is used."))
        .arg(Arg::with_name("cache")
            .short("c")
            .long("cache")
            .takes_value(true)
            .help("The path of the cache to save some data. If blank, then de compiled value of the variable cache_path is used."))
        .arg(Arg::with_name("number")
            .short("n")
            .long("number")
            .takes_value(true)
            .help("The query number. You can see the query number in the queries.rs file, and search a paper by its name to determine the query number."))
        .arg(Arg::with_name("output")
            .short("o")
            .takes_value(true)
            .help("Name of the file to save results. This file is going to be created at /output. Defaults to query_[n].csv"))
        .get_matches();
        

    // get OPTIONS values 

    let query_number: usize = matches.value_of("number").unwrap_or("0").parse().unwrap();

    if query_number <= 0 {
        println!("please provide a query number.")
    }

    dataset_path = matches.value_of("dataset").unwrap_or(dataset_path);

    cache_path = matches.value_of("cache").unwrap_or(cache_path);

    let alternative_output_file: &str = &format!("output/query_{}.csv", query_number);

    let output_file = matches.value_of("output").unwrap_or(alternative_output_file);

    
    // initialize database 
    let database = Djanco::from_spec(
        dataset_path, cache_path,
        timestamp!(March 2021),
        store!(JavaScript), // Technical Debt: specify language?
        Log::new(Verbosity::Debug)
    ).expect("Error initializing Djanco!");

    
    // perform query
    arr_queries[query_number-1](database, output_file);
}
