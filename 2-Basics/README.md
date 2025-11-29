# Rust Basics

## Hello World

1. Erstellen Sie an beliebiger Stelle auf Ihrem Rechner einen Ordner mit dem Namen "basic-tutorial"
2. Öffnen Sie diesen Ordner mit *Visual Studio Code* über die Menüleiste *File -> Open Folder*
3. Bestätigen Sie das Popup mit "Yes, I trust the authors".
4. Öffnen Sie eine Konsole mit der Tasten-Kombination [Strg] + [Shift] + [P] und dem Kommando "Create New Terminal". Als Profil wählen Sie "PowerShell".
5. Geben Sie in die Shell folgendes Kommando ein:
   ```
   cargo init
   ```
   Ein minimales Projekt wird erstellt.
6. Klicken Sie im Projekt-Baum auf der linken Seite die Datei `src/main.rs` an.
7. Klicken Sie Auf `Run` oberhalb der Funktion `fn main()`.
8. Verifizieren Sie dass der Terminal "Hello, world!" ausgibt.

## Variablen in Rust: Ownership, Borrowing & Lifetimes

### Mutablity

```rust
fn main() {
    // `a` is nur lesbar, aber per Shadowing kann `a`
    // neu definiert werden.
    let a = 1;
    let a = a + 1;
    println!("{a} should be 2");

    // Alternative: `a` kann schreibbar definiert werden
    let mut a = 1;
    a = a + 1;
    println!("{a} should be 2");
}
```

### Lifetimes und Shadowing

```rust
fn main() {
    // Definiert die Variable a mit Wert 1
    let a = 1;

    // Klammern erzeugen einen neuen Scope
    {
        let a = 2;
        println!("{a} should be 2");
        // Die Lifetime vom inneren `a` endet hier!
    }

    // Das äußere `a` bleibt unverändert
    println!("{a} should be 1");
}
```

### Ein einfacher Funktionsaufruf

```rust
// Input: 2 Parameter, je vom Typ i32
// Output: i32
fn add(a: i32, b: i32) -> i32 {
    // Kein Semikolon am Ende -> das Ergebnis des Ausdrucks
    // wird von der Funktion zurück gegeben
    a + b
}

fn main() {
    // Definiert zwei Variablen
    let a = 1;
    let b = 2;

    // Ruft die Funktion `add` mit den beiden Variablen auf
    // und speichert das Ergebnis in der Variablen `c`
    let c = add(a, b);

    // Gibt das Ergebnis auf der Konsole aus
    println!("{a} + {b} = {c}");
}
```

### Ein Funktionsaufruf mit Schreibreferenz

```rust
fn accumulate(acc: &mut i32, val: i32) {
    // Per * kann eine Referenz dereferenziert werden
    *acc = *acc + val;
}

fn main() {
    // Definiert eine veränderbare Variable
    let mut a = 41;

    // Erzeugt eine Schreibreferenz für die Variable `a`
    // und ruf `accumulate` auf.
    accumulate(&mut a, 1);

    // Gibt den Wert der Variablen `a` aus
    println!("Sinn des Lebens: {a}");
}
```

## Unit-Testing

Unit-Tests können in derselben Datei untergebracht werden!

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("Sinn des Lebens: {}", add(21, 21));
}

#[cfg(test)]
mod tests {
    use super::add;

    #[test]
    fn should_add_two_numbers() {
        assert_eq!(add(1, 2), 3);
        assert_ne!(add(1, 2), 4);
        assert!(add(1, 2) == 3);
    }
}
```

## Datenstrukturen und Traits von Rust

```rust
// Definiert eine Struktur
struct Adder {
    // Die Struktur hat ein einzelnes Feld vom Typ i32
    acc: i32,
}

// Fügt der Struktur Methoden hinzu
impl Adder {
    // Erzeugt eine neue `Adder` Instanz
    fn init(initial_val: i32) -> Self {
        Self { acc: initial_val }
    }

    // Addiert `val` auf die Adder-Instant `self`
    fn acc(&mut self, val: i32) {
        self.acc += val;
    }

    // Konsumiert die Adder-Instanz `self` und gibt
    // den akkumulierten Wert zurück
    fn finish(self) -> i32 {
        self.acc
    }
}

// Implementiert den Trait `Display`
// Damit ist es möglich `Adder` per Format-String ausgeben zu lassen
impl std::fmt::Display for Adder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Current value is {}", self.acc)
    }
}

// Implementiert den Trait `AddAssign`
// Damit kann die `+=`-Operation auf dem Struct ausgeführt werden
impl std::ops::AddAssign<i32> for Adder {
    fn add_assign(&mut self, rhs: i32) {
        self.acc(rhs);
    }
}

fn main() {
    let mut adder = Adder::init(43);

    // Addieren per Methoden-Aufruf
    adder.acc(1);
    println!("Adder state: {}", adder);

    // Addieren per AddAssign
    adder += 2;
    let final_value = adder.finish();
    println!("Accumulated value: {final_value}");
}
```

## Kontroll-Strukturen

```rust
fn funky_add(a: i32, b: i32) -> Option<i32> {
    // if-Strukturen können Verte zurückgeben
    let return_value = if a == 42 {
        None
    } else {
        Some(a + b)
    };

    return_value
}

fn main() {
    // STDIN zum lesen öffnen
    let stdin = std::io::stdin();

    // ... und zum zeilenweise Lesen vorbereiten
    let mut line_reader = stdin.lines();

    // So lange lesen, bis ein gültiger i32-Wert eingegeben wurde
    let number = loop {
        // Nächste Zeile lesen ...
        let line = line_reader.next()
            .expect("Unexpected EOF")
            .expect("Cannot read stdin");

        // Zeile versuchen in i32 zu wandeln
        let parse_result = line.parse::<i32>();

        // Ergebnis auswerten
        match parse_result {
            // ... war erfolgreich -> break kann den Wert zurück geben
            // und die `loop` verlassen
            Ok(number) => break number,
            // ... Fehler geben eine Hinweis
            Err(_) => println!("Keine gültige Nummer!"),
        }
    };

    // if-Strukturen können Werte zurückgeben
    let sum = if number == 42 {
        Option::None
    } else {
        Option::Some(number + 1)
    };

    match sum {
        Some(sum) => println!("Nächste Zahl: {sum}"),
        None => println!("Wir haben kein Ergebnis zurückbekommen :-/"),
    }
}
```

Ebenfalls interferessant:
* `for` können s.g. Iteratoren lesen
* `while` ist eine `loop` mit Bedingung

## Externe Crates

Paket-Quellen:
* [crates.io](https://crates.io) ist die offizielle Quelle für externe Crates
* [lib.rs](https://lib.rs) hat die bessere Suche, greift aber auf dieselbe Datenbank zu
  * Tipp: Das Stichwort `no_std` führt Crates auf, die ohne Standard-Library auskommen. Praktisch für Embedded!

Installation eines Pakets, bspw.:
```
cargo add example-crate
```

Nutzung einer Crate:
```rust
use example_crate;

fn main() {
    example_crate::example_fn();
}
```

## Module

### Innerhalb einer Datei

```rust
mod mod_a {
    fn a() {}
}

mod mod_b {
    fn b() {}
}

fn main() {
    mod_a::a();
    mod_b::b();
}
```

### In separaten Dateien

* `mod_a.rs`:
  ```rust
  fn a() {}
  ```

* `mod_b.rs`:
  ```rust
  fn b() {}
  ```

* `main.rs`:
  ```rust
  mod mod_a;
  mod mod_b;
  
  fn main() {
      mod_a::a();
      mod_b::b();
  }
  ```
