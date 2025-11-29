# Rust Embedded Demo

**Ziel:** Die ESP32-C6-DevBoards spielen eine Regenbogen-Animation über die Onboard-RGB-LED ab und synchronisieren sich über WiFi-Broadcast-Nachrichten mit anderen DevBoards in Funkreichweite.

Die Farbe im Farbspektrum wird durch eine `u16`-Variable repräsentiert. Da sowohl der minimale Wert `0` als auch der maximale Wert `u16::MAX = 65535` die Farbe Rot abbilden, ist ein Überlauf der Variablen erwünscht, um ein nahtloses, zyklisches Farbspektrum zu ermöglichen.

![Farbspektrum](./docs/color.svg)

Wenn zwei DevBoards ihren aktuellen Farbton austauschen, können sie sich anhand der folgenden Regeln annähern und auf diese Weise synchronisieren:

- Liegt der empfangene Farbton im Spektrum voraus, wird die Animation leicht beschleunigt.
- Liegt der empfangene Farbton im Spektrum zurück, wird die Animation leicht verlangsamt.
- Stimmen eigener und empfangener Farbton überein, bleibt die Abspielgeschwindigkeit unverändert.

## Workspace

Im Workshop-Verzeichnis `3-Demo` befinden sich hilfreiche Code-Schnipsel, die dabei unterstützen, den Workshop in der vorgesehenen Zeit durchzuführen. Im Verlauf des Workshops werden diese Schnipsel schrittweise miteinander kombiniert, sodass am Ende eine Firmware mit der oben beschriebenen Funktionalität entsteht.

Den eigentlichen Code schreiben Sie in der Datei `3-Demo/src/lib.rs`.

## Aufgaben

### Entwicklungsumgebung einrichten

1. Schließen Sie das bereitgestellte DevBoard an Ihr Laptop an.
2. **Nur Windows:** Weisen Sie dem USB-Gerät den richtigen Treiber zu:
   * [zadig](https://github.com/pbatard/libwdi/releases/download/v1.5.1/zadig-2.9.exe) starten
   * **Options** → **List All Devices** aktivieren
   * **USB JTAG/serial debug unit (Interface 2)** auswählen
   * Im Drop-down des mittleren Buttons die Aktion **Install Driver** auswählen. Der Button zeigt danach **Replace Driver** an.
   * Mit **Replace Driver** den Treiber installieren.
3. Öffnen Sie in VS Code ein Terminal im Verzeichnis `3-Demo`.
4. Führen Sie `cargo run` aus. Der Code wird kompiliert, geflasht und gestartet. In `src/lib.rs` befindet sich der Beispielcode, der *"Hallo Workshop!"* ausgibt.
   Beenden Sie das Logging mit **Strg + C**.
5. Führen Sie `cargo doc` aus. Dadurch wird der gesamte Projektcode dokumentiert, einschließlich aller eingebundenen Bibliotheken. Dies kann etwas Zeit dauern.
   Am Ende erscheint ein Link zur lokal erzeugten Dokumentation. Öffnen Sie diesen im Browser.
   *(Hinweis für Teilnehmende mit Code-Server: Das Kommando `doc-link` gibt einen direkt nutzbaren Link aus.)*

### LED Blinky

Ziel dieser Übung ist es, die Onboard-LED zum Blinken zu bringen.

Die zuvor erzeugte Dokumentation enthält das Modul `led`. Ein Klick darauf zeigt Beispielcode zur Nutzung des `Led`-Moduls.

**Aufgabe 1-1:**
Kopieren Sie den Beispielcode in `src/lib.rs` – beachten Sie dabei den Hinweis am Anfang der Datei, bestimmte Bereiche unverändert zu lassen.
Führen Sie den Code aus und prüfen Sie, ob die LED blinkt.

**Aufgabe 1-2:**
Passen Sie den Code so an, dass die LED doppelt so schnell blinkt.
*Hinweis:* Die `Duration`-Struktur besitzt eine Methode zur Angabe von Millisekunden; der Language Server hilft Ihnen beim Auffinden.

**Sonder-Aufgabe 1-3:** *(für die Schnellen)*
Starten Sie die Animation erst, wenn der **BOOT**-Taster auf dem DevBoard gedrückt wird. Laut Pinout ist dieser an **GPIO9** angeschlossen und zieht den Pin beim Drücken auf GND.
Hinweise zur Implementierung:
* `esp_hal::gpio::Input` ist ein geeigneter Treibertyp zum Einlesen von GPIOs.
* Der Pin muss als `Pull::Up` konfiguriert werden.

![Pinout](./docs/pinout.png)

### LED Regenbogen-Animation

Ziel dieser Übung ist es, die Onboard-LED eine Regenbogen-Animation ausführen zu lassen.

Die Dokumentation enthält das Modul `rainbow`, mit dem sich eine Regenbogen-Animation auf einer `Led` darstellen lässt.

**Aufgabe 2-1:**
Starten Sie die Animation mit `rainbow::start_animation()` und übergeben Sie die passenden Parameter.
*Hinweis:* Das Argument `spawner` ermöglicht es der Funktion, die Animation in einem eigenen Task auszuführen.

**Aufgabe 2-2:**
Lassen Sie sich über den `HueReporter`, den `start_animation()` zurückgibt, den aktuellen Farbton per `defmt::info!()` ausgeben.
*Hinweise:*
* Der `HueReporter` stellt eine passende `async fn` bereit.
* Um Werte auszulesen, rufen Sie die Funktion in einer Endlosschleife mit `.await` auf.


### WiFi-Kommunikation

Ziel dieser Übung ist es, Broadcast-Nachrichten über WiFi zu senden und zu empfangen.

Das Modul `net` in der Dokumentation enthält eine Demo für das Netzwerkmodul.

**Aufgabe 3-1:**
Kopieren Sie die Demo (und sichern Sie Ihren bisherigen Code in einem neuen Editor-Tab) und führen Sie sie aus.
Sie sollten nun Nachrichten der anderen Workshop-Teilnehmenden empfangen.

**Aufgabe 3-2:**
Schließen Sie sich mit 1–2 Tischnachbarn zusammen und vereinbaren Sie ein neues Secret.
Senden Sie sich anschließend statt einer Zeichenkette eine Zahl zu.
*Hinweise:*
* Das Modul kann beliebige Datentypen senden und empfangen, solange diese über `serde` serialisierbar bzw. deserialisierbar sind.
* Wichtig: Alle Netzwerk-Knoten müssen denselben Datentyp verwenden.

### Zusammenbau der finalen Firmware

In diesem Kapitel werden die zuvor erarbeiteten Programmteile kombiniert.
Ziel ist es, dass alle DevBoards ihre Animation miteinander synchronisieren.

**Aufgabe 4-1:**
Anstatt feste Werte zu senden, sollen nun `HueReporter` und `HueAdjuster` aus *Aufgabe 2-2* miteinander kommunizieren.
Passen Sie das Programm so an, dass Werte aus dem `HueReporter` über `NetTX` ausgesendet werden.
Empfangene Werte aus `NetRX` sollen an den `HueAdjuster` weitergegeben werden.
Verwenden Sie das von der Workshop-Leitung bereitgestellte Secret.

**Sonder-Aufgabe 4-2:** *(für die Kongress-Woche)*
Lassen Sie Ihr Board laufen und schauen Sie, ob Sie andere Kursteilnehmende anhand ihrer LED-Animation wiederfinden – vielleicht ergibt sich ein gemeinsamer Kaffee. ☕🙂

<details>

Finale Firmware:

```rust
use embassy_futures::select::{Either, select};

/// Application ... this is your playground!
pub async fn main(spawner: Spawner, peripherals: Peripherals) {
    // Start RTOS
    rtos::start(peripherals.TIMG0, peripherals.SW_INTERRUPT);

    // Start network stack
    let key = b"Rust rocks!";
    let (net_rx, net_tx) = net::start_net::<u16>(&spawner, peripherals.WIFI, key);

    // Start animation
    let led = led::Led::new(peripherals.SPI2, peripherals.GPIO8);
    let (hue_reporter, hue_adjuster) = rainbow::start_animation(&spawner, led);

    loop {
        match select(net_rx.recv(), hue_reporter.recv()).await {
            Either::First(hue) => {
                hue_adjuster.adjust(hue).await;
            }
            Either::Second(hue) => {
                net_tx.send(hue).await;
            }
        }
    }
}
```

</details>
