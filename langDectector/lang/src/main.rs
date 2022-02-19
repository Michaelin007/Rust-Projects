use whatlang::{detect, Lang, Script};

fn what_language(mut x: String) {
    let text = &x;

    let info = detect(text).unwrap();
    println!("Language type {}", info.lang());
    println!("Language Script {}", info.script());
    println!("System confidence {}", info.confidence());
    println!("System reliability {}", info.is_reliable());
    /*
    assert_eq!(info.lang(), Lang::Epo);
    assert_eq!(info.script(), Script::Latin);
    assert_eq!(info.confidence(), 1.0);
    assert!(info.is_reliable());
    */
}

fn main() {
    let alice = String::from(
        "Ĉu vi ne volas eklerni Esperanton? Bonvolu! Estas unu de la plej bonaj aferoj!",
    );
    what_language(alice);
}
