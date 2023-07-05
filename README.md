# apppass

[![Crates.io Link](https://crates.io/crates/apppass)](https://crates.io/crates/apppass)

`apppass` is an amazing command-line application that allows you to generate highly secure passwords for the applications you desire, storing them in a registry that you can access from the same CLI using the corresponding command.

# Getting Started

If you don't have Rust installed, don't worry, you can download the available release from the repository.

On the other hand, if you have Rust installed, you can use Cargo to obtain and use the application as follows:

```
$ cargo install apppass
```

Then generate a password application that you want:

```
$ ./apppass --app gmail or ./apppass -a gmail
Password generated and saved for the application: gmail
```

then if you want to see the generated password for the `gmail` app you can execute the command

```
$ ./apppass --get gmail or ./apppass -g gmail
Application_Name: gmail
Password: aB1nwWQyyu2rts7xc3vh90hGk0amlt
```

in the case that you have more one app registered you can execute the following command:

```
$ ./apppass --list or ./apppass -l
Application_Name: github_credential
Password: JsHx7YX4jAaH4L54uBKoNbuHd59ABO

Application_Name: gmail
Password: aB1nwWQyyu2rts7xc3vh90hGk0amlt
```
