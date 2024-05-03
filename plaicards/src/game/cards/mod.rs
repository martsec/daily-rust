use serde::{Deserialize, Serialize};

mod containers;

pub use containers::{Deck, DeckEmptyError, Hand};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Card {
    Adversary {
        title: String,
        description: String,
        strength: i8,
        effect: CardEffect,
    },
    Buzzword {
        title: String,
        description: String,
        strength: i8,
        effect: CardEffect,
    },
    UseCase {
        title: String,
        description: String,
        strength: i8,
        effect: CardEffect,
    },
    Special {
        title: String,
        description: String,
        effect: CardEffect,
    },
    MarketEvent {
        title: String,
        description: String,
        effect: CardEffect,
    },
}

impl Card {
    pub fn title(&self) -> String {
        match self {
            Card::Adversary { title, .. } => title,
            Card::Buzzword { title, .. } => title,
            Card::UseCase { title, .. } => title,
            Card::Special { title, .. } => title,
            Card::MarketEvent { title, .. } => title,
        }
        .into()
    }

    pub fn ctype(&self) -> String {
        match self {
            Card::Adversary { .. } => "Adversary",
            Card::Buzzword { .. } => "Buzzword",
            Card::UseCase { .. } => "UseCase",
            Card::Special { .. } => "Special",
            Card::MarketEvent { .. } => "MarketEvent",
        }
        .into()
    }
    pub fn effect(&self) -> String {
        match self {
            Card::Adversary { effect, .. } => effect,
            Card::Buzzword { effect, .. } => effect,
            Card::UseCase { effect, .. } => effect,
            Card::Special { effect, .. } => effect,
            Card::MarketEvent { effect, .. } => effect,
        }
        .to_string()
    }

    pub fn description(&self) -> String {
        match self {
            Card::Adversary { description, .. } => description,
            Card::Buzzword { description, .. } => description,
            Card::UseCase { description, .. } => description,
            Card::Special { description, .. } => description,
            Card::MarketEvent { description, .. } => description,
        }
        .into()
    }
}

/// This enum contains all possible effects in the game
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum CardEffect {
    NoEffect,

    // Battle Effects
    DiscardBuzzwords,
    PlusTwoVsData,
    PlusTwoVsDeceptive,
    PlusFourVsData,
    PlusOneData,
    PlusOnePython,
    PlusTwoVsManagers,
    PlusTwoBuzzwords,
    PlusThreeCEOs,
    RemovesEffect,
    DiscardOne,
    DiscardBuzzwordsRival,
    DiscardTwo,
    DiscardOneEach,
    CannotDiscard,
    DiscardThreeDrawTwo,
    DiscardThree,

    DiscardAttack,
    AllDrawFour,
    AllDiscardFour,
    Antitrust,
    CardsToNextPlayer,
    ChangeHands,
    AllDiscardOne,
    FourCardVc,
    DrawTwo,
    DrawThree,
    ReviveCard,
    SpyPlayer,
    StealCat,
    Steal2Cards,
    StopEffect,
    StopAttack,
}

impl ToString for CardEffect {
    fn to_string(&self) -> String {
        match self {
            CardEffect::NoEffect => "",
            CardEffect::PlusTwoVsData => "+2 against data cards",
            CardEffect::DiscardBuzzwords => "Discard all buzzword cards plaied",
            CardEffect::PlusTwoVsDeceptive => "+2 against a deceptive Use-case",
            CardEffect::PlusFourVsData => "+4 against data cards",
            CardEffect::RemovesEffect => "Removes all card effects ",
            CardEffect::DiscardOne => "Discard 1 of your rival's plaied cards",
            CardEffect::DiscardBuzzwordsRival => "Your rival discards all buzzword cards they plaied",
            CardEffect::DiscardTwo => "Discard 2 of your rival's plaied cards",
            CardEffect::PlusOneData => "+1 with other data cards",
            CardEffect::PlusOnePython => "+1 with python",
            CardEffect::DiscardOneEach => "Randomly discard 1 plaied card each",
            CardEffect::CannotDiscard => "Your plaied cards cannot be discarded",
            CardEffect::DiscardThreeDrawTwo => "Discard 3 cards, draw 2",
            CardEffect::DiscardThree => "Discard 3 cards",
            CardEffect::DiscardAttack => "Discard all your attack cards",
            CardEffect::AllDrawFour => "Everybody draws 4 cards",
            CardEffect::AllDiscardFour => "Everybody discards 4 cards",
            CardEffect::AllDiscardOne => "Everybody discards 1 card",
            CardEffect::CardsToNextPlayer => "All plaiers give their cards to the next plaier. Cannot be blocked.",
            CardEffect::Antitrust => "All plaiers with more than 9 cards discard 10 cards ",
            CardEffect::ChangeHands => "Change hands with another plaier",
            CardEffect::FourCardVc => "Do a 4 card VC funding",
            CardEffect::DrawTwo => "Draw 2 cards from the deck",
            CardEffect::DrawThree => "Draw 3 cards from the deck",
            CardEffect::ReviveCard => "Revive any card from the discard pile",
            CardEffect::SpyPlayer => "Spy any plAIer's hand and steal one card",
            CardEffect::StealCat => "Steal 1 card from a plAIer. 4 if used as counter for the 'CEO has requested this'. Use it any time.",
            CardEffect::Steal2Cards => "Steal 2 cards from a plAIer",
            CardEffect::StopEffect => "plAI anytime to stop any effect",
            CardEffect::StopAttack => "plAI anytime to stop an attack",
            CardEffect::PlusTwoVsManagers => "+2 against  CEOs and Managers",
            CardEffect::PlusTwoBuzzwords => "+2 to all buzzwords",
            CardEffect::PlusThreeCEOs => "+3 against CEOs and Companies",
        }.to_string()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BasicCard {
    pub title: String,
    pub effect: Option<CardEffect>,
}

#[must_use]
pub fn get_cards_available() -> Vec<Card> {
    use Card::{Adversary, Buzzword, MarketEvent, Special, UseCase};
    vec![
        Adversary {
            title: "nyob (NGO)".to_string(),
            description: "Fighting against giants with GDPR.".to_string(),
            strength: 1,
            effect: CardEffect::PlusTwoVsData,
        },
        Adversary {
            title: "100x ROI".to_string(),
            description: "You put 10€ and get 1000€ back. You always contribute to society speculating, right? Right? ".to_string(),
            strength: 1,
            effect: CardEffect::NoEffect,
        },
        Adversary {
            title: "Consultancy company".to_string(),
            description: "In a single project, one consultancy it's OK, two it's too much and three it's just chaos.".to_string(),
            strength: 1,
            effect: CardEffect::NoEffect,
        },
        Adversary {
            title: "Middle manager".to_string(),
            description: "Trying to scoop water out of the pool with a teaspoon.".to_string(),
            strength: 1,
            effect: CardEffect::NoEffect,
        },
        Adversary {
            title: "Twitter Mob".to_string(),
            description: "With a new name, now without controversies, hate, sexism, racism or insults. GG WP EZ. JK.".to_string(),
            strength: 1,
            effect: CardEffect::NoEffect,
        },
        Adversary {
            title: "New Sheriff".to_string(),
            description: "Ready to save the town with zero trust, bullish estimations, sign offs, weekly deadlines, CoC and nanomanagement.".to_string(),
            strength: 1,
            effect: CardEffect::DiscardBuzzwords,
        },
        Adversary {
            title: "xNet (NGO)".to_string(),
            description: "Technopolitics for the new era. Fighting for schools and students with open tech.".to_string(),
            strength: 2,
            effect: CardEffect::PlusTwoVsDeceptive,
        },
        Adversary {
            title: "Regulatory bodies".to_string(),
            description: "They are taking their time, but they are doing more and more to mitigate harm.".to_string(),
            strength: 2,
            effect: CardEffect::PlusFourVsData,
        },
        Adversary {
            title: "200 poorly paid outsiders".to_string(),
            description: "Intelligence yes, artificial no. Cost: probably less than developing your AI but you can suffer a leak if you don't pay them well enough.".to_string(),
            strength: 2,
            effect: CardEffect::NoEffect,
        },
        Adversary {
            title: "AI regulation".to_string(),
            description: "It can go anywhere... And it will please no-one.".to_string(),
            strength: 2,
            effect: CardEffect::NoEffect,
        },
        Adversary {
            title: "Double CEOs".to_string(),
            description: "Double trouble. ".to_string(),
            strength: 2,
            effect: CardEffect::NoEffect,
        },
        Adversary {
            title: "Human Rights".to_string(),
            description: "What was this again? Ah, that thing we agreed on that we forget exists if there is no oil to be seized.".to_string(),
            strength: 2,
            effect: CardEffect::NoEffect,
        },
        Adversary {
            title: "Pendrive Found".to_string(),
            description: "What will it contain? Cat pictures? For sure there is nothing phishy in it...".to_string(),
            strength: 2,
            effect: CardEffect::NoEffect,
        },
        Adversary {
            title: "Tech Debt".to_string(),
            description: "It's that critical code written 2 years ago, no one understands the dark magic behind it.".to_string(),
            strength: 2,
            effect: CardEffect::RemovesEffect,
        },
        Adversary {
            title: "Tech International".to_string(),
            description: "Workers united against the monopolies of knowledge and decisions. They protest against your killer drone program.".to_string(),
            strength: 2,
            effect: CardEffect::NoEffect,
        },
        Adversary {
            title: "'Expert' CEO".to_string(),
            description: "They want all buzzwords in: AI, BigData, NFT, LLM, Quantum ML...".to_string(),
            strength: 3,
            effect: CardEffect::NoEffect,
        },
        Adversary {
            title: "Daily Standups".to_string(),
            description: "They should be 15 min, have 30 min scheduled and last 2h".to_string(),
            strength: 3,
            effect: CardEffect::NoEffect,
        },
        Adversary {
            title: "DeepFake".to_string(),
            description: "You got played! You thought that call from a politician was real despite being generated by an external actor.".to_string(),
            strength: 3,
            effect: CardEffect::NoEffect,
        },
        Adversary {
            title: "EFF (NGO)".to_string(),
            description: "One of many that without its efforts would leave us in a darker web.".to_string(),
            strength: 3,
            effect: CardEffect::NoEffect,
        },
        Adversary {
            title: "Rich white straight señoro".to_string(),
            description: "He has enough money to buy judges, media and your startup! Maybe racist, misogynist and wants you to work 20h/day.".to_string(),
            strength: 3,
            effect: CardEffect::NoEffect,
        },
        Adversary {
            title: "Wikileaks ".to_string(),
            description: "You know those files that show illegal practices? Yep, they are no longer secret.".to_string(),
            strength: 3,
            effect: CardEffect::NoEffect,
        },
        Adversary {
            title: "Cloud is down".to_string(),
            description: "That happens. Half of the internet does not work today and you are affected.".to_string(),
            strength: 4,
            effect: CardEffect::NoEffect,
        },
        Adversary {
            title: "Global Scale".to_string(),
            description: "You are two technical people but your model has to support half the planet for 50 €/month.".to_string(),
            strength: 4,
            effect: CardEffect::NoEffect,
        },
        Adversary {
            title: "Tech Giant".to_string(),
            description: "Their motto was don't be evil. They removed it. Now why would that be?".to_string(),
            strength: 5,
            effect: CardEffect::NoEffect,
        },
        Adversary {
            title: "Toxic Manager".to_string(),
            description: "He overpromised and now you have all the pressure to deliver 50% of it on time and there is a resignation party.".to_string(),
            strength: 5,
            effect: CardEffect::NoEffect,
        },
        Adversary {
            title: "Soshana Zuboff".to_string(),
            description: "Author of The Era of Surveillance Capitalism: The Fight for a Human Future at the New Frontier of Power.".to_string(),
            strength: 6,
            effect: CardEffect::NoEffect,
        },
        Buzzword {
            title: "Accountability".to_string(),
            description: "Show me that the tech is going to be fair and legal before you put it out.".to_string(),
            strength: -1,
            effect: CardEffect::DiscardOne,
        },
        Buzzword {
            title: "GDPR fine".to_string(),
            description: "Let's be honest, it's low and a small percentage of the damage done.".to_string(),
            strength: -1,
            effect: CardEffect::DiscardOne,
        },
        Buzzword {
            title: "User Centric".to_string(),
            description: "Focused on selling all user data. ".to_string(),
            strength: -1,
            effect: CardEffect::DiscardOne,
        },
        Buzzword {
            title: "Data Outliers".to_string(),
            description: "If someone has 10 phones and you have 0, then the average is 5 phones each.".to_string(),
            strength: -1,
            effect: CardEffect::DiscardBuzzwordsRival,
        },
        Buzzword {
            title: "Investor protection".to_string(),
            description: "You have business angels protecting you.".to_string(),
            strength: -2,
            effect: CardEffect::DiscardTwo,
        },
        Buzzword {
            title: "Trolley dilemma".to_string(),
            description: "Your AI solves it. Every human dies.".to_string(),
            strength: -2,
            effect: CardEffect::DiscardTwo,
        },
        Buzzword {
            title: "Voice Recognition".to_string(),
            description: "With your voice you can order in my shop. You don't even need to use your computer at all!".to_string(),
            strength: -2,
            effect: CardEffect::DiscardTwo,
        },
        Buzzword {
            title: "More Data".to_string(),
            description: "You just need more data to improve you AI's poor results. Or you can accept it does not work. ".to_string(),
            strength: 1,
            effect: CardEffect::PlusOneData,
        },
        Buzzword {
            title: "Artificial Intelligence".to_string(),
            description: "Despite widespread understanding, AI does not resemble our human way of thinking. ".to_string(),
            strength: 1,
            effect: CardEffect::PlusOnePython,
        },
        Buzzword {
            title: "Machine Learning".to_string(),
            description: "Statistics on algorithms on steroids learning from patterns.".to_string(),
            strength: 1,
            effect: CardEffect::PlusOnePython,
        },
        Buzzword {
            title: "NLP".to_string(),
            description: "I can process language, write like Shakespeare or Cervantes, and even advance in quantum electrodynamics.".to_string(),
            strength: 1,
            effect: CardEffect::PlusOnePython,
        },
        Buzzword {
            title: "Ethical Oversight board".to_string(),
            description: "Internal review organ with zero power, in Facebook's case.".to_string(),
            strength: 1,
            effect: CardEffect::DiscardOneEach,
        },
        Buzzword {
            title: "Unicorn".to_string(),
            description: "Your new hire is a data scientist, data engineer, a tech influencer and now an analytics engineer too.".to_string(),
            strength: 1,
            effect: CardEffect::CannotDiscard,
        },
        Buzzword {
            title: "Much More Data".to_string(),
            description: "When more data is not enough for your hungry training. ".to_string(),
            strength: 2,
            effect: CardEffect::PlusOneData,
        },
        Buzzword {
            title: "Neural Network".to_string(),
            description: "Your brain in my chip since 1943. Now with more computing power.".to_string(),
            strength: 2,
            effect: CardEffect::PlusOnePython,
        },
        Buzzword {
            title: "Deep Learning".to_string(),
            description: "Neural networks but with fancier names like keras, tensorflow, pytorch...".to_string(),
            strength: 2,
            effect: CardEffect::NoEffect,
        },
        Buzzword {
            title: "hugginface".to_string(),
            description: "Every AI practitioner wants to hug them for the great job they are doing.".to_string(),
            strength: 2,
            effect: CardEffect::PlusOnePython,
        },
        Buzzword {
            title: "Coded Bias".to_string(),
            description: "When your phone cannot detect you face it's because more crimes are predicted in your neighbourhood.".to_string(),
            strength: 2,
            effect: CardEffect::DiscardOneEach,
        },
        Buzzword {
            title: "Even More Data".to_string(),
            description: "You are an even bigger data hoarder than the tech monopolies.".to_string(),
            strength: 3,
            effect: CardEffect::PlusOneData,
        },
        Buzzword {
            title: "pandas and scikit Learn".to_string(),
            description: "Old, trusty and maybe a bit too much backwards compatible. Your data scientist's best friends.".to_string(),
            strength: 3,
            effect: CardEffect::NoEffect,
        },
        Buzzword {
            title: "Python".to_string(),
            description: "The snake introducing you to this world while playing snake jazz.".to_string(),
            strength: 3,
            effect: CardEffect::NoEffect,
        },
        Buzzword {
            title: "Python".to_string(),
            description: "The snake introducing you to this world while playing snake jazz.".to_string(),
            strength: 3,
            effect: CardEffect::NoEffect,
        },
        Buzzword {
            title: "Statistics".to_string(),
            description: "Old, but still rocks. It can put your Startup to sleep if you ignore it in favor of the fancy algorithms.".to_string(),
            strength: 3,
            effect: CardEffect::NoEffect,
        },
        MarketEvent {
            title: "Mass Layoffs".to_string(),
            description: "Ignite the hearts of venture capitalists by constantly firing your employees.".to_string(),
            effect: CardEffect::DiscardThreeDrawTwo,
        },
        MarketEvent {
            title: "Cloud invoice".to_string(),
            description: "When clouds don't bring rain it means you have to pay.".to_string(),
            effect: CardEffect::DiscardThree,
        },
        MarketEvent {
            title: "Workers Union".to_string(),
            description: "You want to bust'em with all the illegal ways you can think of.".to_string(),
            effect: CardEffect::DiscardAttack,
        },
        MarketEvent {
            title: "Hype".to_string(),
            description: "Your marketing efforts paid off and now you are near the peak of inflated expectations.".to_string(),
            effect: CardEffect::AllDrawFour,
        },
        MarketEvent {
            title: "Dot-com crash".to_string(),
            description: "It's the 2000 crisis again. Brace your phone and laptop".to_string(),
            effect: CardEffect::AllDiscardFour,
        },
        MarketEvent {
            title: "Machine Learning Operations".to_string(),
            description: "They think it's easy to deploy a model to production. Reality says it costs everyone... a card!".to_string(),
            effect: CardEffect::AllDiscardOne,
        },
        Special {
            title: "International data transfer".to_string(),
            description: "All data passes through Fort Meade.".to_string(),
            effect: CardEffect::CardsToNextPlayer,
        },
        Special {
            title: "Antitrust".to_string(),
            description: "You are not too big to fail.*\n\n* Your lobbyist might change this sentence.".to_string(),
            effect: CardEffect::Antitrust,
        },
        Special {
            title: "Antitrust".to_string(),
            description: "You are not too big to fail.*\n\n* Your lobbyist might change this sentence.".to_string(),
            effect: CardEffect::Antitrust,
        },
        Special {
            title: "Antitrust".to_string(),
            description: "You are not too big to fail.*\n\n* Your lobbyist might change this sentence.".to_string(),
            effect: CardEffect::Antitrust,
        },
        Special {
            title: "Antitrust".to_string(),
            description: "You are not too big to fail.*\n\n* Your lobbyist might change this sentence.".to_string(),
            effect: CardEffect::Antitrust,
        },
        Special {
            title: "Antitrust".to_string(),
            description: "You are not too big to fail.*\n\n* Your lobbyist might change this sentence.".to_string(),
            effect: CardEffect::Antitrust,
        },
        Special {
            title: "Talent exchange".to_string(),
            description: "That's how juniors become seniors.".to_string(),
            effect: CardEffect::ChangeHands,
        },
        Special {
            title: "Talent exchange".to_string(),
            description: "That's how juniors become seniors.".to_string(),
            effect: CardEffect::ChangeHands,
        },
        Special {
            title: "Press coverage".to_string(),
            description: "You pray to the 'gods' of the longtermism cult to make it rain.".to_string(),
            effect: CardEffect::FourCardVc,
        },
        Special {
            title: "Killer drones".to_string(),
            description: "Congrats, you have a great new contract!".to_string(),
            effect: CardEffect::DrawTwo,
        },
        Special {
            title: "VC funding".to_string(),
            description: "You have impressed the AI ML GI BD VC AGI DOOM community.".to_string(),
            effect: CardEffect::DrawThree,
        },
        Special {
            title: "10x data scientist".to_string(),
            description: "Do those exist? (Spoiler: you are one of them!)".to_string(),
            effect: CardEffect::ReviveCard,
        },
        Special {
            title: "10x engineer".to_string(),
            description: "Do those exist? (Spoiler: you are one of them!)".to_string(),
            effect: CardEffect::ReviveCard,
        },
        Special {
            title: "Financial Data".to_string(),
            description: "What you bought will tell us if you are pregnant before even you know it.".to_string(),
            effect: CardEffect::SpyPlayer,
        },
        Special {
            title: "Global espionage".to_string(),
            description: "Sponsored by your friendly Human Rights government.".to_string(),
            effect: CardEffect::SpyPlayer,
        },
        Special {
            title: "Global espionage".to_string(),
            description: "Sponsored by your friendly Human Rights government.".to_string(),
            effect: CardEffect::SpyPlayer,
        },
        Special {
            title: "Surveillance".to_string(),
            description: "Walking the narrow line of spying on people and keeping everything secret makes for a distinct power imbalance. But you already knew this, didn't you?".to_string(),
            effect: CardEffect::SpyPlayer,
        },
        Special {
            title: "CEO's friend".to_string(),
            description: "Last minute tasks have no effect on you when you are the CEO's bestie.".to_string(),
            effect: CardEffect::StealCat,
        },
        Special {
            title: "AI bot army".to_string(),
            description: "Publicize a distraction with AI-written messages on social media.".to_string(),
            effect: CardEffect::Steal2Cards,
        },
        Special {
            title: "Board Chaos".to_string(),
            description: "While their executives fired each other it's now your opportunity to hire.".to_string(),
            effect: CardEffect::Steal2Cards,
        },
        Special {
            title: "Great Password".to_string(),
            description: "Your rivals were pwned due to their admin password being 'password'. ".to_string(),
            effect: CardEffect::Steal2Cards,
        },
        Special {
            title: "Ransomware".to_string(),
            description: "Your rival opens an email and... SURPRISE! Their assets are now yours.".to_string(),
            effect: CardEffect::Steal2Cards,
        },
        Special {
            title: "The CEO has requested this".to_string(),
            description: "All requests come at 6 PM and are for later today.".to_string(),
            effect: CardEffect::Steal2Cards,
        },
        Special {
            title: "The CEO has requested this".to_string(),
            description: "All requests come at 6 PM and are for later today.".to_string(),
            effect: CardEffect::Steal2Cards,
        },
        Special {
            title: "arXiv.org ".to_string(),
            description: "Where great and bad articles reside. But... can you distinguish them? ".to_string(),
            effect: CardEffect::StopEffect,
        },
        Special {
            title: "Privacy".to_string(),
            description: "Ah! That which may no longer exist but is essential to our personal growth as individuals.".to_string(),
            effect: CardEffect::StopEffect,
        },
        Special {
            title: "Privacy".to_string(),
            description: "Ah! That which may no longer exist but is essential to our personal growth as individuals.".to_string(),
            effect: CardEffect::StopEffect,
        },
        Special {
            title: "Terms and Conditions".to_string(),
            description: "Accept selling your soul to the devil without reading a single word".to_string(),
            effect: CardEffect::StopEffect,
        },
        Special {
            title: "Bias".to_string(),
            description: "Big tech tells us their machines are perfect. We need to stop this bullshit.".to_string(),
            effect: CardEffect::StopAttack,
        },
        Special {
            title: "Ethics".to_string(),
            description: "We need time to stop and think. To answer how we can do good, minimize harm, respect human autonomy and be fair and just.".to_string(),
            effect: CardEffect::StopAttack,
        },
        Special {
            title: "Security".to_string(),
            description: "Cyber security, military security, private security, cartel security, job security. You choose the field!".to_string(),
            effect: CardEffect::StopAttack,
        },
        UseCase {
            title: "Chatbot".to_string(),
            description: "You trained it with internet forums and now it's racist and far-right.".to_string(),
            strength: 1,
            effect: CardEffect::NoEffect,
        },
        UseCase {
            title: "Impersonate deceased".to_string(),
            description: "Your grandma now recommends products from Amazonia.".to_string(),
            strength: 1,
            effect: CardEffect::NoEffect,
        },
        UseCase {
            title: "Selling smoke".to_string(),
            description: "Your purpose is to automate snake oil selling with AI generated products, descriptions, images, marketing strategies, everything.".to_string(),
            strength: 1,
            effect: CardEffect::NoEffect,
        },
        UseCase {
            title: "Crowdfunding".to_string(),
            description: "Don't depend on Big Capital. Your sole purpose is the same as this game. Steal Branderson's record.".to_string(),
            strength: 2,
            effect: CardEffect::NoEffect,
        },
        UseCase {
            title: "Discrimination".to_string(),
            description: "We may or may not intentionally train our AI to discriminate based on historical data. But we'll hide it regardless.".to_string(),
            strength: 2,
            effect: CardEffect::NoEffect,
        },
        UseCase {
            title: "Predict crimes".to_string(),
            description: "None detected in rich neighbourhoods and you don't see what's wrong with that.".to_string(),
            strength: 2,
            effect: CardEffect::NoEffect,
        },
        UseCase {
            title: "Select employees".to_string(),
            description: "Since it's based on historical data, only white men will be accepted for high paying jobs.".to_string(),
            strength: 2,
            effect: CardEffect::NoEffect,
        },
        UseCase {
            title: "Stock trading".to_string(),
            description: "Get rich making people poor. Now with AI avoiding the Black-Scholes catastrophe.".to_string(),
            strength: 2,
            effect: CardEffect::NoEffect,
        },
        UseCase {
            title: "Win art competitions".to_string(),
            description: "Generate images and videos from random text for internet amazement. Like black cats plaiing this game.".to_string(),
            strength: 2,
            effect: CardEffect::NoEffect,
        },
        UseCase {
            title: "Generate DeepFakes".to_string(),
            description: "Don't you want to start WWIII?".to_string(),
            strength: 3,
            effect: CardEffect::PlusTwoVsManagers,
        },
        UseCase {
            title: "Fix climate".to_string(),
            description: "The planet? Greenwashing? I just see the green of money.".to_string(),
            strength: 3,
            effect: CardEffect::PlusTwoBuzzwords,
        },
        UseCase {
            title: "Facial Recognition".to_string(),
            description: "Your repressive government's favorite tool. Ah, it can also be used to unlock phones too.".to_string(),
            strength: 3,
            effect: CardEffect::NoEffect,
        },
        UseCase {
            title: "Internet Of Things".to_string(),
            description: "The promised future since 1982. The smart thing (and you) will learn all about the owners.".to_string(),
            strength: 3,
            effect: CardEffect::NoEffect,
        },
        UseCase {
            title: "Manipulate elections".to_string(),
            description: "Who? We? What proof do you have? - Cambridge Analytica".to_string(),
            strength: 3,
            effect: CardEffect::NoEffect,
        },
        UseCase {
            title: "Predict criminals using faces".to_string(),
            description: "Pseudo-science from the 30s. Why it's still used nowadays astonishes us.".to_string(),
            strength: 4,
            effect: CardEffect::NoEffect,
        },
        UseCase {
            title: "Self-driving car".to_string(),
            description: "Who does it have to put in danger?".to_string(),
            strength: 4,
            effect: CardEffect::NoEffect,
        },
        UseCase {
            title: "Sentient AI".to_string(),
            description: "It's for sure a marketing strategy. All media is onboard, but what does it mean to be sentient? ".to_string(),
            strength: 4,
            effect: CardEffect::NoEffect,
        },
        UseCase {
            title: "Write black mirror episodes".to_string(),
            description: "A favorite of the content-creation industry. Will it be like Westworld? ".to_string(),
            strength: 5,
            effect: CardEffect::NoEffect,
        },
        UseCase {
            title: "Auto Moderation".to_string(),
            description: "You say content moderation is  easy and your AI will solve it. In reality, it's highly complex even for humans.".to_string(),
            strength: 6,
            effect: CardEffect::NoEffect,
        },
        UseCase {
            title: "OSINT monitoring".to_string(),
            description: "Monitor Corporations and Governments to reverse the power imbalance.".to_string(),
            strength: 7,
            effect: CardEffect::PlusThreeCEOs,
        },

    ]
}
