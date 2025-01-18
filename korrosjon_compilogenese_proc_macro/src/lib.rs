use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Prob" => "Err",
        "Bra" => "Ok",
        "Streng" => "String",
        "Ordbok" => "HashMap",
        "Standard" => "Default",
        "Feil" => "Error",
        "Kanskje" => "Option",
        "Noen" => "Some",
        "Ingenting" => "None",
        "Resultat" => "Result",
        "Selv" => "Self",
        "skrivlinje" => "println",
        "bryt" => "break",
        "asynkron" => "async",
        "avvent" => "await",
        "løkke" => "loop",
        "flytte" => "move",
        "eske" => "crate",
        "uoppnåelig_kode" => "unreachable_code",
        "som" => "as",
        "konstant" => "const",
        "egenskap" => "trait",
        "utrygt" => "unsafe",
        "av" => "in",
        "fra" => "from",
        "dynamisk" => "dyn",
        "pakk_opp" => "unwrap",
        "standard" => "default",
        "som_ref" => "as_ref",
        "iu" => "io",
        "ekstern" => "extern",
        "falsk" => "false",
        "funksjon" => "fn",
        "over" => "super",
        "sett_inn" => "insert",
        "les" => "get",
        "tillat" => "allow",
        "panikk" | "dritt" | "faen" | "ops" | "uff" => "panic",
        "modul" => "mod",
        "foranderlig" => "mut",
        "ny" => "new",
        "der" => "where",
        "for" => "for",
        "ta_eller_sett_inn_med" => "get_or_insert_with",
        "hoved" => "main",
        "offentlig" => "pub",
        "ingen" => None?,
        "retur" => "return",
        "implementer" => "impl",
        "referanse" => "ref",
        "sammenlign" => "match",
        "hvis" => "if",
        "ellers" => "else",
        "selv" => "self",
        "la" => "let",
        "statisk" => "static",
        "struktur" => "struct",
        "forvent" => "expect",
        "imens" => "while",
        "bruk" => "use",
        "til" => "into",
        "sant" => "true",
        "oppregning" => "enum",
        "Gruppe" => "Group",
        "Identifikator" => "Ident",
        "TokenFlyt" => "TokenStream",
        "TokenTre" => "TokenTree",
        "til_streng" => "to_string",
        "som_en_streng" => "as_str",
        "omfang" => "span",
        "Vektor" => "Vec",
        "strøm" => "stream",
        "dytt" => "push",
        "utvid" => "extend",
        "skilletegn" => "delimiter",
        "Tegnsetting" => "Punct",
        "Bokstavelig" => "Literal",
        "prosedyremakro" => "proc_macro",
        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn korrosjon(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
