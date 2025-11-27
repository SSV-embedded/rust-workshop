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
    *acc = *acc + 1;
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
