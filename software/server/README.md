# audio-controller-host

This code is the code that runs on the windows/linux host. It is made to be as cross-platform as possible. It's built on the tokio framework. This will be especially useful in the future, when we want to make a ui so the user can configure functionality easily. This then allows us to run the ui in a seperate thread. Most likely built on slint. But this is a long-term goal, short-term is getting a basic functionality by manually editing config files.
