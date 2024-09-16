# Onecanvas
Onecanvas is a 3d canvas for multiple people to collaborate and make art in 3d, as well as explore what others have made. It stores all data in MongoDB, handles it using Rust, and is sent to a Svelte webapp that uses WebXR to allow AR viewing of the art that's been made. It's able to handle multiple people using it at once, allowing collaboration.

This was a project for the 2024 HackWesTX in the Texas Tech Innovation Center. I'd classify this project as very incomplete, and there's currently no push to finish it. If you would like to host it yourself, you are welcome to, but there are some things you need to know first:
1. This runs as a web app, meaning it uses WebXR features that are only available on android, and only available on versions of chrome after 2021
2. The site begins assuming that the place your phone is at is the center of the universe. We originally wanted it to be global and use GPS, but GPS is very innacurate and browsers don't have access to exact location under normal circumstances
3. It is very slow, especially when a lot of lines exist, or one very long line exists. Because the server doesn't stream the data, and instead sends it all in one packet, the app sometimes spends a full minute waiting to be sent all the data
4. Certificate signing is broken, and it currently can only use self-sign, meaning many browsers will give a warning before allowing you to continue. Easy way to fix this is to run it through a proxy server like Nginx or Caddy
5. The client is very basic. There's no way to choose your color (it is currently random), and there's no way to specify a thickness (limitation with threejs lines)

You're welcome to fix any of these issues on your own if you would like, but there are no plans for official support. Both me (Mallow, frontend dev) and Yosh (backend dev) have other more important projects for our degrees. If you do make substantial improvements, I will consider merging.