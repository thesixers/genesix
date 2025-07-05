# 📘 Genesix Programming Language - README

Welcome to **Genesix** — a modern, clean, and expressive programming language that blends the elegance of Python with the flexibility of JavaScript, while offering the control and speed of low-level languages.

---

## ✨ Overview

Genesix is:

- Object-oriented at its core (all data types are objects)
- Simple to read and write
- Flexible for beginners and powerful for advanced developers
- Suitable for backend, scripting, and system-level programming

---

## 📦 Data Types & Built-In Methods

### 1. `string`

```genesix
name = "Joe"
```

#### Methods:

- `str.length`
- `str.upper()`
- `str.lower()`
- `str.trim()`
- `str.split(delimiter)`
- `str.replace(old, new)`
- `str.includes(substr)`
- `str.indexOf(substr)`
- `str.startsWith(prefix)`
- `str.endsWith(suffix)`
- `str.toArray()` → array of characters

---

### 2. `number`

```genesix
age = 30
```

#### Methods:

- `num.toString()`
- `num.toFixed(n)`
- `num.toPrecision(n)`
- `num.toExponential(n)`
- `num.toInt()`
- `num.toFloat()`

---

### 3. `boolean`

```genesix
isLoggedIn = true
```

---

### 4. `Array`

```genesix
users = ["joe", "emily"]
```

#### Methods:

- `arr.length`
- `arr.push(item)`
- `arr.pop()`
- `arr.map(func)`
- `arr.filter(func)`
- `arr.find(func)`
- `arr.indexOf(item)`
- `arr.includes(item)`
- `arr.join(delimiter)`
- `arr.slice(start, end)`
- `arr.toString()`
- `arr.toObject(keyField, valueField)`

---

### 5. `Object`

```genesix
user = {name: "Joe", age: 25}
```

#### Methods:

- `obj.keys()`
- `obj.values()`
- `obj.entries()`
- `obj.hasKey(key)`
- `obj.toArray()`
- `obj.toJSON()`
- `obj.merge(otherObj)`

---

### 6. `Function`

First-class, supports closures.

```genesix
init greet(name):
    log(`Hello, ${name}`)
```

---

### 7. `null` / `undefined`

```genesix
x = null
y = undefined
```

---

### 8. `Date`

```genesix
d = Date.now()
```

---

### 9. `Enum`

```genesix
enum Role { Admin, User, Guest }
```

---

### 10. `Symbol`

```genesix
sym = Symbol("key")
```

---

### 11. `BigInt`

```genesix
big = BigInt("90071992547409911234567890")
```

---

## 📌 Variables

```genesix
name = "Joe"            # mutable
*age = 30                # immutable
name: string = "Joe"     # optional typing
```

---

## ⚙️ Functions

```genesix
init greet(name):
    log(`Hello, ${name}`)

add = (x, y) =>: return x + y

init printAll(...names):
    log(names)
```

- Supports: closures, default params, rest params, return
- No `const` needed for arrow functions

---

## 🧠 Conditionals

```genesix
if (age > 18):
    log("Adult")
elif (age == 18):
    log("Just became adult")
else:
    log("Minor")

result = age > 18 ? "Yes" : "No"
```

---

## 🔁 Loops

```genesix
for (i = 0; i < 5; i++):
    log(i)

for (item in items):
    log(item)

for ({item, i} in items):
    log(`${i}: ${item}`)

range(1, 5):
    log("loop")

while (x < 10):
    log(x)
    x += 1
```

---

## 🧱 Classes

```genesix
class Animal:
    _init_(name):
        self.name = name

    speak():
        log(self.name)

class Dog extends Animal:
    speak():
        log(self.name + " barks")

@secure
class AdminPanel:
    _init_():
        ...

class Math:
    fixed PI = 3.14
    fixed add(x, y): return x + y
```

- Decorators supported for classes & functions
- `fixed` = static field/method
- `_init_` = constructor

---

## 🧪 Error Handling

```genesix
try:
    risky()
catch (ValidationError err):
    log(err.message)
finally:
    log("Done")

throw "Something broke"
throw {type: "BadInput", message: "Missing name"}
```

---

## 🔌 Import System

```genesix
get fs from "fs"
get os from "os"
get utils from "./utils.gx"
get file from input("Which module?")
```

---

## 📂 File System Module (`fs`)

```genesix
get fs from "fs"
```

| Method                   | Purpose           |
| ------------------------ | ----------------- |
| `fs.read(path)`          | Read file content |
| `fs.write(path, data)`   | Write to file     |
| `fs.append(path, data)`  | Append to file    |
| `fs.exists(path)`        | File exists?      |
| `fs.delete(path)`        | Delete file       |
| `fs.list(path)`          | List directory    |
| `fs.mkdir(path)`         | Create directory  |
| `fs.rmdir(path)`         | Remove directory  |
| `fs.copyFile(src, dest)` | Copy file         |
| `fs.move(old, new)`      | Move file/folder  |
| `fs.rename(old, new)`    | Rename file       |
| `fs.stat(path)`          | File info         |
| `fs.clear(path)`         | Empty file        |
| `fs.isDir(path)`         | Is directory?     |

---

## ⚙️ OS Module (`os`)

```genesix
get os from "os"
```

#### Methods:

- `os.platform()`
- `os.arch()`
- `os.env()`
- `os.cwd()`
- `os.exit(code)`

---

## 🌐 HTTP Module (Coming Soon)

```genesix
data = await http.get("https://api.example.com")
```

---

## 🧵 Async/Await

```genesix
data = await fetchData()
```

- Works inside any function (regular or arrow)
- No `async` keyword needed — runtime handles it

---

## 🧠 Extra Features

- Destructuring: `{name, age} = user`, `[a, b] = list`
- Spread: `newList = [...oldList]`, `copy = {...user}`
- Optional chaining: `user?.profile?.email`
- Nullish coalescing: `name = input() ?? "Guest"`
- Named parameters in functions: `init save({name, age})`

---

## 🗨️ Logging

```genesix
log("Hello, world")
```

---

## 🧾 Comments

```genesix
<* This is a comment *>
```

---

## 🧠 Memory Model

- All variables are mutable by default
- Use `*name = value` to declare immutable values

---

## 📜 Template Strings

```genesix
name = "Joe"
log(`Hello, ${name}!`)
```

---

## 🎉 That's Genesix!

You now have a full-featured, readable, powerful, object-oriented, async-ready language built from the best parts of Python and JavaScript.

Time to build it! 💻

