# 1 Getting started with git

## 1.1 Installing git

### 1.1.1 Windows

On windows, we use WSL, which gives us a virtual linux kernel with which to

1. Install [Windows Subsystem for Linux (WSL)](https://learn.microsoft.com/en-us/windows/wsl/install).
2. Reboot.
3. Run WSL.
4. Follow the instructions for Linux.

### 1.1.2 Linux

If you are on any mainstream Linux distro, your package manager will include a package for git. This guide is written on the assumption you are using Ubuntu, if you are using a different distro the specific package manager instructions will be different, but google "(your distro) install git" should return something useful.

1. Open a terminal. If you don't know how to do this, try `Ctrl + Shift + T`.
2. On Debian-based distributions like Ubuntu, you can run

   ```
   $ sudo apt update
   $ sudo apt install git
   ```

   to update the package database and install git.

3. Verify the installation by running

   ```
   $ git --version
   ```

### 1.1.3 MacOS

The easiest way to install git on MacOS is by using [Homebrew](https://brew.sh).

1. If you don't have Homebrew, [install it now](https://brew.sh).
2. Open a terminal, e.g. by pressing `Ctrl + Opt + Shift + T`.
3. Run the command

   ```
   $ brew install git
   ```

4. Verify the installation worked by running

   ```
   $ git --version
   ```

## 1.2 Setting up GitHub

To work with the Software Team, you don't only need git, but also GitHub.

1. If you don't already have one, create a [GitHub](https://github.com) account.
2. In your terminal (WSL on Windows), run the following commands to make sure your contributions are attirbuted correctly:

   ```
   $ git config --global user.name <Your Name>
   $ git config --global user.email <your@github.email>
   ```

   Make sure you are using the same email adress as on your GitHub account.

3. Set up SSH for connecting to GitHub:
   1. `ssh-keygen -t ed25519 -C "your_email@example.com"`
   2. Accept the default save location
   3. Give it a password or none
   4. Add it to github following [this guide](https://docs.github.com/en/authentication/connecting-to-github-with-ssh/adding-a-new-ssh-key-to-your-github-account)
   5. Test your connection by running `ssh -T git@github.com`
      1. You will see something like
      ```
      > The authenticity of host 'github.com (IP ADDRESS)' can't be established.
      > ED25519 key fingerprint is SHA256:+DiY3wvvV6TuJJhbpZisF/zLDA0zPMSvHdkr4UvCOqU.
      > Are you sure you want to continue connecting (yes/no)?
      ```
      2. Answer yes
      3. You should then see this if you have set it up properly
      ```
      > Hi USERNAME! You've successfully authenticated, but GitHub does not
      > provide shell access.
      ```
