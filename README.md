# apppass

[![Crates.io Link](https://crates.io/crates/apppass)](https://crates.io/crates/apppass)

In the world of software development, security is of paramount importance. Generating strong passwords is a common need for developers and users alike. If you're a programmer looking for a reliable and convenient way to generate secure passwords for your applications, look no further than `apppass`. This command-line application, written in Rust, offers a simple yet powerful solution to address this requirement.

# What is apppass?
`apppass` is an amazing command-line application built with Rust. It empowers developers and users to generate highly secure passwords for their applications. With `apppass`, you can store these passwords in a secure registry, accessible directly from the command line interface (CLI) using intuitive commands.

# Getting Started

If you don't have Rust installed on your machine, fret not! You can easily download the latest release of `apppass` from the repository. This ensures that you can quickly get started without worrying about the installation process.

For those who already have Rust installed, you can leverage Cargo, Rust's package manager, to obtain and utilize `apppass` effortlessly. Simply run the following command to install `apppass`:

```
$ cargo install apppass
```

Once you have `apppass` installed, generating passwords is a breeze. Let's say you want to create a password for your Gmail application. Simply execute the following command:

```
$ ./apppass --app gmail or ./apppass -a gmail
Password generated and saved for the application: gmail
```
After generating the password, `apppass` will automatically save it for the specified application, in this case, `gmail`. You'll receive a confirmation message indicating that the password has been successfully stored.

Upon executing this command, `apppass` will display the application name (`gmail`) and its corresponding password, providing you with the necessary information you need.

```
$ ./apppass --get gmail or ./apppass -g gmail
Application_Name: gmail
Password: aB1nwWQyyu2rts7xc3vh90hGk0amlt
```

If you have multiple applications registered with `apppass`, you can easily list them using the following command:

This command will display a list of the registered applications along with their respective passwords, allowing you to keep track of your credentials efficiently.

```
$ ./apppass --list or ./apppass -l
Application_Name: github_credential
Password: JsHx7YX4jAaH4L54uBKoNbuHd59ABO

Application_Name: gmail
Password: aB1nwWQyyu2rts7xc3vh90hGk0amlt
```

# Conclusion:
`apppass` is a remarkable Rust-based command-line application that simplifies the process of generating secure passwords for your applications. With its user-friendly interface and secure password storage capabilities, `apppass` offers a convenient solution for developers and users seeking robust password management. By leveraging the power of Rust, `apppass` ensures reliability, performance, and the highest standards of security. Give `apppass` a try today and enhance the security of your applications with ease.
