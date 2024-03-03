use std::collections::HashMap;


#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Genre {
    Male, Female
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct CreditNomine {
    pub a: i32,
    pub b: i32,
    pub c: i32,
    pub d: i32,
}

pub type CreditHash = HashMap<i32, CreditNomine>;
pub type GeneralCreditHash = HashMap<Genre, CreditHash>;


#[derive(Debug)]
pub struct CreditData {
    pub min_credits: GeneralCreditHash,
    pub max_credits: GeneralCreditHash,
}

pub fn load_data() -> CreditData {
    let mut min_credits: GeneralCreditHash = HashMap::new();
    let mut max_credits: GeneralCreditHash = HashMap::new();

    let mut male_min_credits: CreditHash = HashMap::new();
    let mut female_min_credits: CreditHash = HashMap::new();

    let mut male_max_credits: CreditHash = HashMap::new();
    let mut female_max_credits: CreditHash = HashMap::new();
    
    male_min_credits.insert(26, CreditNomine {
        a: 100,
        b: 1000,
        c: 400,
        d: 400
    });

    male_min_credits.insert(27, CreditNomine {
        a: 400,
        b: 600,
        c: 200,
        d: 300
    });

    male_min_credits.insert(28, CreditNomine {
        a: 900,
        b: 1000,
        c: 200,
        d: 500
    });

    male_min_credits.insert(29, CreditNomine {
        a: 100,
        b: 1000,
        c: 1000,
        d: 900
    });

    male_min_credits.insert(30, CreditNomine {
        a: 600,
        b: 1000,
        c: 600,
        d: 1000
    });

    min_credits.insert(Genre::Male, male_min_credits);

    female_min_credits.insert(24, CreditNomine {
        a: 800,
        b: 800,
        c: 200,
        d: 500
    });

    female_min_credits.insert(25, CreditNomine {
        a: 800,
        b: 700,
        c: 900,
        d: 1000
    });
    

    min_credits.insert(Genre::Female, female_min_credits);

    male_max_credits.insert(26, CreditNomine {
        a: 4900,
        b: 4700,
        c: 5000,
        d: 4400
    });

    male_max_credits.insert(27, CreditNomine {
        a: 4700,
        b: 4400,
        c: 4700,
        d: 4700
    });

    male_max_credits.insert(28, CreditNomine {
        a: 4600,
        b: 5000,
        c: 5000,
        d: 4300
    });

    male_max_credits.insert(29, CreditNomine {
        a: 4600,
        b: 4400,
        c: 4200,
        d: 4900
    });

    male_max_credits.insert(30, CreditNomine {
        a: 4500,
        b: 4900,
        c: 4600,
        d: 4300
    });

    max_credits.insert(Genre::Male, male_max_credits);

    female_max_credits.insert(24, CreditNomine {
        a: 4000,
        b: 4700,
        c: 4600,
        d: 5000
    });

    female_max_credits.insert(25, CreditNomine {
        a: 4200,
        b: 4200,
        c: 4900,
        d: 4900
    });

    female_max_credits.insert(26, CreditNomine {
        a: 4100,
        b: 4500,
        c: 4600,
        d: 4700
    });

    female_max_credits.insert(27, CreditNomine {
        a: 4200,
        b: 4300,
        c: 4700,
        d: 5000
    });

    female_max_credits.insert(28, CreditNomine {
        a: 4500,
        b: 4400,
        c: 4000,
        d: 4300
    });

    max_credits.insert(Genre::Female, female_max_credits);

    CreditData {
        min_credits,
        max_credits
    }
}
