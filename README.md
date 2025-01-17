<p align="center">
  <img src="https://github.com/cosmic-utils/Chronos/blob/main/res/icons/hicolor/scalable/apps/com.francescogaglione.chronos.svg" alt="Logo" width="200px">
</p>

# Chronos

**Chronos** is a simple and intuitive Pomodoro timer for Linux, built using **libcosmic**, the libraries underlying the **Cosmic** desktop environment. It is designed to help you enhance your productivity by effectively managing work time and breaks using the Pomodoro technique.

## Features

- Standard Pomodoro timer with configurable duration for work sessions and breaks.
- Minimalist interface integrated with the Cosmic desktop environment.
- Ability to customize work and break times in the settings.

## Installation

The project supports three installation modes:

1. **.deb**: Download the .deb package from the GitHub repository and install it using your system's package manager.

### Installation from Source

#### Steps

To install the application from source, follow these steps:

```bash
# Clone the repository
git clone https://github.com/cosmic-utils/Chronos

# Change directory to the project folder
cd Chronos

# Build Release version (for much better performance)
just build-release

# Install
sudo just install
```

## Configuration

The **Chronos** settings allow you to customize:

- Duration of work sessions.
- Duration of short and long breaks.
- Number of work sessions before a long break.

All settings are accessible through the user interface.

## Screenshots

Here’s a preview of **Chronos**:

### Pomodoro

![Screenshot of the main page](./screenshots/pomodoro-page.png)

### Settings

![Screenshot of the settings page](./screenshots/pomodoro-settings.png)

## Contributing

Contributions are welcome! If you’d like to contribute to the project, please follow these steps:

1. Fork the repository.
2. Create a new branch for your changes: `git checkout -b your-branch-name`.
3. Make your changes and commit them: `git commit -m 'Description of changes'`.
4. Push your changes: `git push origin your-branch-name`.
5. Open a pull request.

## Future Goals

1. System notifications
2. Notification settings
3. Activity history

## License

Distributed under the **GPL-3.0** license. See the [LICENSE](./LICENSE) file for more details.
