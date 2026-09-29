# 🐸 Bouncy Koen - Crossy Road Edition

Een 3D HTML5 Crossy Road-stijl game gebouwd met Three.js, met Koen in de hoofdrol en een oer-Hollandse twist!

![Bouncy Koen](assets/koen.jpg)

---

## 🌟 Features

- **3D Voxel Koen**: Compleet met zijn kenmerkende kuif, gezichtsuitdrukking, donkere confetti-shirt en komische *squash & stretch* hop-animaties.
- **Oer-Hollandse Banen**:
  - 🌳 **Parken**: Veilige grasstroken met tulpen en bomen.
  - 🚲 **Fietspaden**: Rood asfalt met snelle houten bakfietsen en stadsfietsers.
  - 🚗 **Autowegen**: Drukke wegen vol auto's en bestelbusjes.
  - 🛶 **Grachten**: Kabbelend water met Amsterdamse rondvaartboten en drijvende vlotten om op te springen.
  - 🚆 **NS Spoorovergangen**: Razendsnelle geel-blauwe dubbeldekkers (VIRM) met knipperende overweglichten.
- **De Brutale Zeemeeuw**: Twijfel je langer dan 5 seconden? Dan duikt er een zeemeeuw uit de lucht om Koen mee te kapen!
- **Besturing**: Pijltjestoetsen / WASD, Swipe of de on-screen D-Pad.
- **Highscore Systeem**: Opgeslagen in de browser (`localStorage`).

---

## 🥔 Pterodactyl / Aardappel Hosting

Deze repo is 100% kant-en-klaar geoptimaliseerd voor Pterodactyl met de **Node.js generic egg**:

- **0 Externe Packages**: Gebruikt puur Node's native `http`, `fs` en `path` modules.
- **Geen build-stap**: Geen `npm install` wachttijd, geen RAM-spikes.
- **~25 MB RAM**: Start in minder dan 50ms en draait soepel op zelfs de kleinste aardappel-servers.
- **Automatische Poort**: Luistert automatisch naar `SERVER_PORT` / `PORT` op `0.0.0.0`.

### Lokale Start
- **Windows**: Dubbelklik op `start.bat`.
- **Node.js**: `node index.js`
