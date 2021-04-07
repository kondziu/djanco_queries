use djanco::*;
use djanco::log::*;
use clap::{Arg, App};

mod lib;

use std;
use std::path::PathBuf;

fn main() {

    // database path to make queries to.
    let dataset_path = "/Users/stefaniemuroyalei/Documents/prague/work/codedj-parasite/example-dataset";
    let cache_path = "/Users/stefaniemuroyalei/Documents/prague/work/codedj-parasite/cache";
    let output_path = "output";

    // vector of queries, every query is added to this vector.
    let arr_queries: Vec< fn (a: &djanco::data::Database, b: &djanco::log::Log, c: &std::path::Path) -> Result<(), std::io::Error>> =
                vec![
                    lib::query1, lib::query2, lib::query3,
                    lib::query4, lib::query5, lib::query6,
                    lib::query7, lib::query8, lib::query9,
                    lib::query10, lib::query11, lib::query12,
                    lib::query13, lib::query14, lib::query15
                ];

    // Define command line flags
    let matches = App::new("Replication of Queries for Various Papers Using Djanco")
        .author("Stefanie Muroya")
        .arg(Arg::new("dataset")
            .short('d')
            .long("dataset")
            .takes_value(true)
            .about("The path of the dataset that contains repos. \
                   If blank, then de compiled value of the constant dataset_path is used."))
        .arg(Arg::new("cache")
            .short('c')
            .long("cache")
            .takes_value(true)
            .about("The path of the cache to save some data. \
                   If blank, then de compiled value of the variable cache_path is used."))
        .arg(Arg::new("number")
            .short('n')
            .long("number")
            .takes_value(true)
            .about("The query number. You can see the query number in the lib file, and \
                   search a paper by its name to determine the query number."))
        .arg(Arg::new("output")
            .short('o')
            .takes_value(true)
            .about("Name of the directory to save results.  \
                   Defaults to ./output"))
        .get_matches();
        

    // get OPTIONS values 

    let query_number: usize = matches.value_of("number").unwrap_or("0").parse().unwrap();
    if query_number <= 0 {
        panic!("please provide a query number.")
    }

    let dataset_path = matches.value_of("dataset").unwrap_or(dataset_path);
    let cache_path = matches.value_of("cache").unwrap_or(cache_path);
    let output_path = matches.value_of("output").unwrap_or(output_path);
    
    // initialize database
    let log =  Log::new(Verbosity::Debug);
    let database = Djanco::from_spec(
        dataset_path, cache_path,
        timestamp!(March 2021),
        store!(JavaScript), // Technical Debt: specify language?
        log.clone()
    ).expect("Error initializing Djanco!");

    // perform query
    arr_queries[query_number-1](&database, &log, &PathBuf::from(output_path)).unwrap();
}
