# apppass 🚀🔒

[![Crates.io Link](https://crates.io/crates/apppass)](https://crates.io/crates/apppass)

`apppass` is a powerful command-line application that allows you to generate, manage, and secure passwords efficiently. With advanced features like temporary passwords (OTP), memorable passwords, import/export, and auto-lock, `apppass` takes password management to the next level. ✨

---

## 🔧 **Installation**

Install `apppass` easily with:

```bash
$ cargo install apppass
```

---

## ✨ **Key Features**

- 🔒 **Secure Password Generation**: Create highly secure random passwords.
- ⏰ **Temporary Passwords (OTP)**: Generate passwords valid for a limited time.
- 🤓 **Memorable Passwords**: Easy-to-remember yet secure passwords.
- 🔄 **Full Password Management**: List, update, delete, import, and export passwords.
- 🕗 **Auto-Lock**: Locks the application after a period of inactivity.
- 📂 **Export/Import**: Exchange passwords via CSV files.

---

## 🚀 **Core Commands**

### 🔒 **Generate a Password**

Create a password for an application:

```bash
$ ./apppass --app gmail
Password generated and saved for the application: gmail
```

Specify the password length:

```bash
$ ./apppass --app github --length 40
Password generated and saved for the application: github
```

---

### 🔍 **Retrieve a Password**

Retrieve the password for an application:

```bash
$ ./apppass --get gmail
Application_Name: gmail
Password: aB1nwWQyyu2rts7xc3vh90hGk0amlt
```

---

### 🔄 **List All Passwords**

Show all registered applications and their passwords:

```bash
$ ./apppass --list
Application_Name: github_credential
Password: JsHx7YX4jAaH4L54uBKoNbuHd59ABO

Application_Name: gmail
Password: aB1nwWQyyu2rts7xc3vh90hGk0amlt
```

---

### ❌ **Delete a Password**

Delete the password for an application:

```bash
$ ./apppass --delete gmail
Application 'gmail' deleted successfully.
```

---

### ♻️ **Update a Password**

Update the password for an application:

```bash
$ ./apppass --update gmail
Password updated for 'gmail'.
```

---

### 📂 **Export Passwords to a CSV File**

Save all your passwords to a file:

```bash
$ ./apppass --export passwords.csv
Passwords exported to 'passwords.csv'.
```

---

### 📂 **Import Passwords from a CSV File**

Import passwords from an existing file:

```bash
$ ./apppass --import passwords.csv
Passwords imported from 'passwords.csv'.
```

---

### ⏰ **Generate a Temporary Password (OTP)**

Create a password that expires after a defined time:

```bash
$ ./apppass --otp MyApp --ttl 300
Temporary Password: 7aB8cD9EfG
Expires at: 2025-01-10 12:00:00
```

---

### 🤓 **Generate a Memorable Password**

Create a secure and easy-to-remember password:

```bash
$ ./apppass --memorizable BlogApp
Memorizable Password for 'BlogApp': Tiger-85-Cloud
```

---

### 🕗 **Set Auto-Lock**

Configure an inactivity period after which the application locks:

```bash
$ ./apppass --lock 60
Auto-lock set to 60 seconds.
```

---

## 🙌 **Contribute**

Have an idea to improve `apppass`? We welcome contributions! You can send a pull request or open an issue on the [GitHub repository](https://github.com/your-username/apppass).

---

## 🚀 **Next Steps**

- Cloud synchronization support.
- Security report generation.
- Integration with other password managers.

---
