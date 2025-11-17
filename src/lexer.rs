use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {

    #[token("$")]
    Dollar,

    #[token("%")]
    Percent,

    #[token(">")]
    Greater,

    #[token("<")]
    Less,

    #[token("#include")]
    Include,

    #[regex("#[0-f]+")]
    Number,

    #[regex("[a-zA-Z0-9]+")]
    Ident,

    #[regex("[a-zA-Z0-9]+:")]
    Label,
    
    #[regex("@[a-zA-Z0-9]+")]
    LabelAdress,

    #[regex("\"([^\"]*)\"")]
    String,

    #[regex("//.*", logos::skip)]
    Comment,
}
