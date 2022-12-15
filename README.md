This repository generates an erroneous clippy error:

```
error: this loop never actually loops
  --> src/main.rs:8:5
   |
8  | /     while let Ok(event) = receiver.recv() {
9  | |         let ServiceEvent::ServiceResolved(info) = event else {
10 | |             // We don't care about other events here
11 | |             continue;
...  |
21 | |         ));
22 | |     }
   | |_____^
   |
   = note: `#[deny(clippy::never_loop)]` on by default
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#never_loop

error: could not compile `rust-clippy-loop-false-positive` due to previous error
```

Using if/let rather than let/else makes the problem go away:

```
diff --git a/src/main.rs b/src/main.rs
index 5941d79..79ff814 100644
--- a/src/main.rs
+++ b/src/main.rs
@@ -10,7 +10,11 @@ fn demo() -> Result<(String, String)> {
             // We don't care about other events here
             continue;
         };
-        let Some(addr) = info.get_addresses().iter().next() else {
+        let mut addr = String::from("");
+        if let Some(addr_inner) = info.get_addresses().iter().next() {
+            // ...
+            addr.push_str(&addr_inner.to_string());
+        } else {
             // No one should ever have zero addresses, but just in case...
             continue;
         };
```
