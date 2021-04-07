use djanco::*;
use djanco::csv::*;
use djanco::log::*;
use djanco::data::*;
use djanco::objects::*;
use djanco::fraction::Fraction;

use djanco_ext::*;

use chrono::{DateTime, Utc, Duration};

// BEGIN MSR20
#[djanco(April, 2021, subsets(Generic))]
// What constitutes Software? An Empirical, Descriptive Study of Artifacts
pub fn query1 (database: &Database, _log: &Log, output: &std::path::Path) -> Result<(), std::io::Error> {
    /* Notes:
     * - ABAP language not supported
     * - ADA language not supported
     */
        
    let target_files = vec![
        Language::ASM, Language::C, Language::Cobol, Language::Cpp,
        Language::CSharp, Language::D, Language::Erlang, Language::Fortran,
        Language::FSharp, Language::Go, Language::Groovy, Language::Java,
        Language::JavaScript, Language::Kotlin, Language::Lua, Language::ObjectiveC,
        Language::OCaml, Language::Perl, Language::PHP, Language::Python, Language::Ruby,
        Language::Scala, Language::Swift
    ];

    database.projects()
        .filter_by(Member(project::Language, target_files))
        .filter_by(Not(project::IsFork))
        .group_by(project::Language)
        .sort_by(project::Stars)
        .sample(Top(1020))
        //.map_into(Select!(project::Stars,project::URL, FromEach(project::Paths, path::Location))) // Just project selection
        .into_csv_in_dir(output, "query_1.csv")

}

#[djanco(April, 2021, subsets(Generic))]
// A Study of Potential Code Borrowing and License Violations in Java Projects on GitHub
pub fn query2(database: &Database, _log: &Log, output: &std::path::Path) -> Result<(), std::io::Error> {
    database.projects()
        .filter_by(Equal(project::Language, Language::Java))
        .filter_by(Not(project::IsFork))
        .filter_by(AtLeast(project::Stars, 50))
        //.map_into(Select!(project::Stars,project::URL, project::License)) // Just project selection
        .into_csv_in_dir(output, "query_2.csv")
}

#[djanco(April, 2021, subsets(Generic))]
// An Empirical Study of Method Chaining in Java
pub fn query3(database: &Database, _log: &Log, output: &std::path::Path) -> Result<(), std::io::Error> {
    // query executed multiple times over a period of time
    database.projects()
        .filter_by(Equal(project::Language, Language::Java))
        .filter_by(Not(project::IsFork))
        .filter_by(AtLeast(project::Created, timestamp!(January 2011))) // code older than 2010
        .sort_by(project::Stars)
        .sample(Top(1000))
        //.map_into(Select!(project::URL,project::Stars)) // Just project selection
        .into_csv_in_dir(output, "query_3.csv")
}

#[djanco(April, 2021, subsets(Generic))]
// Capture the Feature Flag: Detecting Feature Flags in Open-Source
pub fn query4(database: &Database, _log: &Log, output: &std::path::Path) -> Result<(), std::io::Error> {
    let flag_regex = regex!("(feature|remove|delete|cleanup).{0,50}(flag|toggle)");

    database.projects()
        .filter_by(Equal(project::Language, Language::Java))
        .filter_by(Not(project::IsFork))
        .filter_by(AtLeast(Count(FromEachIf(project::Commits, Matches(commit::Message, flag_regex.clone()))), 10))
        .sort_by(Count(FromEachIf(project::Commits, Matches(commit::Message, flag_regex.clone()))))
        // .map_into(       // Just project selection
        //     Select!(
        //         project::URL,
        //         FromEachIf(project::Commits, Matches(commit::Message, flag_regex.clone()))
        //     )
        // )
        .into_csv_in_dir(output, "query_4.csv")
}

// #[djanco(April, 2021, subsets(Generic))] // Not interesting
// Detecting and Characterizing Bots that Commit Code
pub fn query5(database: &Database, _log: &Log, output: &std::path::Path) -> Result<(), std::io::Error> {
    let email_regex = regex!(".*(bot).*@.*");

    database.projects()
        // .map_into(
        //     Select!(
        //         project::URL,
        //         FromEachIf(
        //             project::Users,
        //             //Or(
        //             Matches(user::Email, email_regex.clone())
        //             //)
        //         )
        //     ))
        .into_csv_in_dir(output, "query_5.csv")
}

#[djanco(April, 2021, subsets(Generic))]
// Developer-Driven Code Smell Prioritization
pub fn query6(database: &Database, _log: &Log, output: &std::path::Path) -> Result<(), std::io::Error> {
    let five_years_ago: DateTime<Utc> = Utc::now() - Duration::weeks(12*4*5);

    database.projects()
        .filter_by(Equal(project::Language, Language::Java))
        .filter_by(AtLeast(Count(FromEachIf(project::Commits, AtLeast(commit::AuthoredTimestamp, five_years_ago.timestamp()))), 1))
        .filter_by(AtLeast(Count(project::Commits), 1000))
        .filter_by(AtLeast(Count(project::Committers), 20))
        .filter_by(AtLeast(Count(FromEachIf(project::Paths, Equal(path::Language, Language::Java ))), 500))
        .sample(Random(9, Seed(1))) 
        .sort_by(Count(FromEachIf(project::Paths, Equal(path::Language, Language::Java )))) 
        //.map_into(Select!(project::URL, project::Size)) // Just project selection
        .into_csv_in_dir(output, "query_6.csv")
}

#[djanco(April, 2021, subsets(Generic))]
// Did You Remember To Test Your Tokens?
pub fn query7(database: &Database, _log: &Log, output: &std::path::Path) -> Result<(), std::io::Error> {
    database.projects()
        .filter_by(Equal(project::Language, Language::Java))
        .filter_by(Not(project::IsFork))
        .filter_by(MoreThan(project::Size, 0)) // non-empty project
        .sample(Random(100000, Seed(1)))  
        //.map_into(Select!(project::URL, project::Size)) // Just project selection
        .into_csv_in_dir(output, "query_7.csv")
}

// #[djanco(April, 2021, subsets(Generic))] // not interesting
// Forking Without Clicking: on How to Identify Software Repository Forks
pub fn query8(database: &Database, _log: &Log, output: &std::path::Path) -> Result<(), std::io::Error> {
    database.projects()
        .into_csv_in_dir(output, "query_8.csv")
}

#[djanco(April, 2021, subsets(Generic))]
// Characterizing and Identifying Composite Refactorings: Concepts, Heuristics and Patterns
pub fn query9(database: &Database, _log: &Log, output: &std::path::Path) -> Result<(), std::io::Error> {
    database.projects()
        .filter_by(
            Not(project::IsFork)
        )
        .filter_by(project::HasIssues) // active issue tracking system
        .filter_by(
            AtLeast(
                Ratio(
                    FromEachIf(
                        project::Paths, Equal(path::Language, Language::Java)
                    ), 
                    project::Paths
                ), 
                Fraction::new(9,10)
            )
        ) // at least 90% of code written in JAVA
        //.sample(Random(48, Seed(1)))
        .into_csv_in_dir(output, "query_9.csv")
}

#[djanco(April, 2021, subsets(Generic))]
// The State of the ML-universe: 10 Years of Artificial Intelligence & Machine Learning Software Development on GitHub
// Collecting a Comparison Set
pub fn query10(database: &Database, _log: &Log, output: &std::path::Path) -> Result<(), std::io::Error> {
    let one_year_ago: DateTime<Utc> = Utc::now() - Duration::weeks(12*4);

    database.projects()
        .filter_by(Not(project::IsFork))
        .filter_by(MoreThan(project::Size, 0)) // non-empty
        .filter_by( 
            Or(
                AtLeast(project::Stars, 5),
                AtLeast(project::Forks, 5)
            )
        ) //popularity
        .filter_by(
            AtLeast(
                Count(
                    FromEachIf(
                        project::Commits, 
                        AtLeast(commit::AuthoredTimestamp, one_year_ago.timestamp())
                    )
                ), 
                1
            )
        )// activity
        .sample(Random(10000, Seed(1)))
        .sort_by(project::Stars)
        .into_csv_in_dir(output, "query_10.csv")
}

#[djanco(April, 2021, subsets(Generic))]
// Using Large-Scale Anomaly Detection on Code to Improve Kotlin Compiler
pub fn query11(database: &Database, _log: &Log, output: &std::path::Path) -> Result<(), std::io::Error> {
    database.projects()
        //.filter_by(Equal(project::Language, Language::Kotlin))
        .filter_by(Not(project::IsFork))
        .filter_by(AtMost(project::Created,timestamp!(March 2018)))
        .into_csv_in_dir(output, "query_11.csv")
}

// END MSR20
// BEGIN MSR19
#[djanco(April, 2021, subsets(Generic))]
// Import2vec Learning Embeddings for Software Libraries
pub fn query12(database: &Database, _log: &Log, output: &std::path::Path) -> Result<(), std::io::Error> {
    let target_files = vec![
        Language::Java, Language::JavaScript, Language::Python,
        Language::Ruby, Language::PHP, Language::CSharp
    ];

    database.projects()
        .filter_by(Member(project::Language, target_files))
        .filter_by(AtLeast(project::Stars, 2))
        .filter_by(Not(project::IsFork))
        .into_csv_in_dir(output, "query_12.csv")
}

#[djanco(April, 2021, subsets(Generic))]
//Semantic Source Code Models Using Identifier Embeddings
pub fn query13(database: &Database, _log: &Log, output: &std::path::Path) -> Result<(), std::io::Error> {
    let target_files = vec![
        Language::Java, Language::Python, Language::PHP, 
        Language::CSharp, Language::C, Language::Cpp
    ];

    database.projects()
        .filter_by(Member(project::Language, target_files))
        .filter_by(Not(project::IsFork))
        .filter_by(AtLeast(project::Stars, 100))
        .sort_by(project::Stars)
        .into_csv_in_dir(output, "query_13.csv")
}

#[djanco(April, 2021, subsets(Generic))]
//Predicting Good Configurations for GitHub and Stack Overflow Topic Models
pub fn query14(database: &Database, _log: &Log, output: &std::path::Path) -> Result<(), std::io::Error> {
    let target_files = vec![
        Language::Java, Language::Python, Language::HTML, 
        Language::CSS, Language::C, Language::Cpp,
        Language::Ruby, Language::JavaScript
    ];

    let readme_regex = regex!("(readme|README)\\.md");
    database.projects()
        .filter_by(Member(project::Language, target_files))
        .filter_by(
            AtLeast(
                Count(
                    FromEachIf(
                        project::Paths, 
                        Matches(path::Location, readme_regex.clone())
                    )
                ), 
                1
            )
        ) // has readme
        .sample(Random(5000, Seed(1))) 
        // .map_into(           // Just project selection
        //     Select!(
        //         project::URL,
        //         FromEachIf(
        //             project::Paths,
        //             Matches(path::Location, readme_regex.clone())
        //         )
        //     ))
        .into_csv_in_dir(output, "query_14.csv")
}

#[djanco(April, 2021, subsets(Generic))]
// A Dataset of Parametric Cryptographic Misuses
pub fn query15(database: &Database, _log: &Log, output: &std::path::Path) -> Result<(), std::io::Error> {
    database.projects()
        .filter_by(
            Equal(project::Language, Language::Java)
        )
        .filter_by(
            MoreThan(project::Stars, 100)
        )
        .filter_by(
            MoreThan(
                Count(project::Commits),
                100
            )
        )
        .filter_by(
            AtLeast(project::Created, timestamp!(July 2015))
        )
        .filter_by(
            AtMost(project::Created, timestamp!(August 2018))
        )
        .into_csv_in_dir(output, "query_15.csv")
}

// END MSR19