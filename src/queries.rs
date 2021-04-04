use djanco::csv::*;
use djanco::*;
use djanco::objects::*;
use chrono::{DateTime, Utc, Duration};
use djanco::fraction::Fraction;


// BEGIN MSR20
pub fn query1 (database: djanco::data::Database, output_file: &str) {
    // Title: What constitutes Software? An Empirical, Descriptive Study of Artifacts

    //Notes: 
        // - ABAP language not supported
        // - ADA language not supported
        
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
        .map_into(Select!(project::Stars,project::URL, FromEach(project::Paths, path::Location)))
        //save output
        .into_csv(output_file)
        .expect("Error saving result");

}

pub fn query2(database: djanco::data::Database, output_file: &str){

    // A Study of Potential Code Borrowing and License Violations in Java Projects on GitHub

    database.projects()
        .filter_by(Equal(project::Language, Language::Java))
        .filter_by(Not(project::IsFork))
        .filter_by(AtLeast(project::Stars, 50))
        .map_into(Select!(project::Stars,project::URL, project::License))
        //save output
        .into_csv(output_file)
        .expect("Error saving result");
}

pub fn query3(database: djanco::data::Database, output_file: &str){

    // An Empirical Study of Method Chaining in Java


    // query executed multiple times over a period of time
    database.projects()
        .filter_by(Equal(project::Language, Language::Java))
        .filter_by(Not(project::IsFork))
        .filter_by(AtLeast(project::Created, timestamp!(January 2011))) // code older than 2010
        .sort_by(project::Stars)
        .sample(Top(1000))
        .map_into(Select!(project::URL,project::Stars))
        //save output
        .into_csv(output_file)
        .expect("Error saving result");

}

pub fn query4(database: djanco::data::Database, output_file: &str){

    // Capture the Feature Flag: Detecting Feature Flags in Open-Source

    let flag_regex = regex!("(feature|remove|delete|cleanup).{0,50}(flag|toggle)");

    database.projects()
        .filter_by(Equal(project::Language, Language::Java))
        .filter_by(Not(project::IsFork))
        .filter_by(AtLeast(Count(FromEachIf(project::Commits, Matches(commit::Message, flag_regex.clone()))), 10))
        .sort_by(Count(FromEachIf(project::Commits, Matches(commit::Message, flag_regex.clone()))))
        .map_into(
            Select!( 
                project::URL,
                FromEachIf(project::Commits, Matches(commit::Message, flag_regex.clone()))
            )
        )
        //save output
        .into_csv(output_file)
        .expect("Error saving result");

}

pub fn query5(database: djanco::data::Database, output_file: &str){

    // Detecting and Characterizing Bots that Commit Code

    let email_regex = regex!(".*(bot).*@.*");

    database.projects()
        .map_into(
            Select!(
                project::URL, 
                FromEachIf(
                    project::Users, 
                    //Or(
                    Matches(user::Email, email_regex.clone())
                    //)
                )
            ))        
        //save output
        .into_csv(output_file)
        .expect("Error saving result");
}

pub fn query6(database: djanco::data::Database, output_file: &str){

    // Developer-Driven Code Smell Prioritization

    let five_years_ago: DateTime<Utc> = Utc::now() - Duration::weeks(12*4*5);

    database.projects()
        .filter_by(Equal(project::Language, Language::Java))
        .filter_by(AtLeast(Count(FromEachIf(project::Commits, AtLeast(commit::AuthoredTimestamp, five_years_ago.timestamp()))), 1))
        .filter_by(AtLeast(Count(project::Commits), 1000))
        .filter_by(AtLeast(Count(project::Committers), 20))
        .filter_by(AtLeast(Count(FromEachIf(project::Paths, Equal(path::Language, Language::Java ))), 500))
        .sample(Random(9, Seed(1))) 
        .sort_by(Count(FromEachIf(project::Paths, Equal(path::Language, Language::Java )))) 
        .map_into(Select!(project::URL, project::Size)) 
        //save output
        .into_csv(output_file)
        .expect("Error saving result");

}

pub fn query7(database: djanco::data::Database, output_file: &str){

    // Did You Remember To Test Your Tokens?

    database.projects()
        .filter_by(Equal(project::Language, Language::Java))
        .filter_by(Not(project::IsFork))
        .filter_by(MoreThan(project::Size, 0)) // non-empty project
        .sample(Random(100000, Seed(1)))  
        .map_into(Select!(project::URL, project::Size)) 
        //save output
        .into_csv(output_file)
        .expect("Error saving result");

}

pub fn query8(database: djanco::data::Database, output_file: &str){

    // Forking Without Clicking: on How to Identify Software Repository Forks

    database.projects()
        //save output
        .into_csv(output_file)
        .expect("Error saving result");
}

pub fn query9(database: djanco::data::Database, output_file: &str){

    // Characterizing and Identifying Composite Refactorings: Concepts, Heuristics and Patterns

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
        //save output
        .into_csv(output_file)
        .expect("Error saving result");
}

pub fn query10(database: djanco::data::Database, output_file: &str){

    // The State of the ML-universe: 10 Years of Artificial Intelligence & Machine Learning Software Development on GitHub
        // Collecting a Comparison Set

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
        .into_csv(output_file)
        .expect("Error saving result");
}

pub fn query11(database: djanco::data::Database, output_file: &str){

    // Using Large-Scale Anomaly Detection on Code to Improve Kotlin Compiler


    database.projects()
        //.filter_by(Equal(project::Language, Language::Kotlin))
        .filter_by(Not(project::IsFork))
        .filter_by(AtMost(project::Created,timestamp!(March 2018)))
        .into_csv(output_file)
        .expect("Error saving result");
}

// END MSR20
// BEGIN MSR19

pub fn query12(database: djanco::data::Database, output_file: &str){

    // Import2vec Learning Embeddings for Software Libraries

    let target_files = vec![
        Language::Java, Language::JavaScript, Language::Python,
        Language::Ruby, Language::PHP, Language::CSharp
    ];

    database.projects()
        .filter_by(Member(project::Language, target_files))
        .filter_by(AtLeast(project::Stars, 2))
        .filter_by(Not(project::IsFork))
        .into_csv(output_file)
        .expect("Error saving result");
    
}

pub fn query13(database: djanco::data::Database, output_file: &str){

    //Semantic Source Code Models Using Identifier Embeddings

    let target_files = vec![
        Language::Java, Language::Python, Language::PHP, 
        Language::CSharp, Language::C, Language::Cpp
    ];

    database.projects()
        .filter_by(Member(project::Language, target_files))
        .filter_by(Not(project::IsFork))
        .filter_by(AtLeast(project::Stars, 100))
        .sort_by(project::Stars)
        .into_csv(output_file)
        .expect("Error saving result");
    
}

pub fn query14(database: djanco::data::Database, output_file: &str){

    //Predicting Good Configurations for GitHub and Stack Overflow Topic Models

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
        .map_into(
            Select!(
                project::URL, 
                FromEachIf(
                    project::Paths, 
                    Matches(path::Location, readme_regex.clone())
                )
            ))  
        .into_csv(output_file)
        .expect("Error saving result");
    
}

pub fn query15(database: djanco::data::Database, output_file: &str){

    // A Dataset of Parametric Cryptographic Misuses

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
        .into_csv(output_file)
        .expect("Error saving result");
    
}




// END MSR19