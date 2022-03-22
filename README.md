# imgrs

[![pipeline status](https://git.geraldwu.com/gerald/omgur/badges/master/pipeline.svg)](https://git.geraldwu.com/gerald/omgur/-/commits/master) 

Imgrs is a free and open-source alternative Imgur front-end focused on privacy.

Inspired by the [Invidious](https://github.com/iv-org/invidious), [Nitter](https://github.com/zedeus/nitter), and [Teddit](https://github.com/teddit-net/teddit) projects.

- No JavaScript or ads
- All requests go through the backend, client never talks to Imgur
- Prevents Imgur from tracking your IP or JavaScript fingerprint
- Lightweight
- Self-hostable

This is a Rust rewrite of a previous Imgur proxy project, [Omgur](https://git.geraldwu.com/gerald/omgur).

## Features roadmap

- [x] Direct image loading via https://i.imgur.com/
- [x] Imgur album loading via https://imgur.com/a/
- [ ] Imgur gallery loading via https://imgur.com/gallery/
- [x] Imgur post loading via https://imgur.com/
- [x] Redis caching for images
- [ ] Proper embedding of videos on albums/galleries (direct loading already works)
- [ ] Mock tests of the functionality
- [ ] Docker image
- [ ] Automatic GitLab CI/CD pipeline

## Far-future roadmap

- [ ] Render comments on Imgur posts
- [ ] Frontpage imgur url form
- [ ] Public API endpoints
