# 🐸 Bouncy Koen - Crossy Road Edition

Een 3D HTML5 Crossy Road-stijl game gebouwd met Three.js, met Koen in de hoofdrol en een oer-Hollandse twist!


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

## Publiceren

De game is een statische website in `docs/`: `index.html`, `js/` en `assets/`. Er is geen build-stap of backend nodig. De paden in `index.html` zijn relatief en werken ook op een GitHub Pages-projectpad (`/bouncykoen/`).

### GitHub Pages (aanbevolen)

1. Push de bestanden naar de `main`-branch.
2. De bestaande Pages-publicatie vanaf `main` **/(root)** blijft werken: `index.html` verwijst automatisch door naar `docs/`. Je kunt in **Settings → Pages → Build and deployment** ook branch **main**, folder **/docs** instellen om de game direct op de homepage te tonen.
3. Open de Pages-URL na publicatie. `CNAME` en `docs/CNAME` configureren de bestaande eigen domeinnaam `bouncykoen.schoolnaam.nl` voor beide Pages-bronnen; zorg dat de DNS voor dat domein naar GitHub Pages wijst. Verwijder beide CNAME-bestanden als je geen eigen domein wilt gebruiken.

### Cloudflare Workers (statische assets)

`wrangler.jsonc` publiceert alleen `docs/` als statische assets zonder Worker-script. Server- en projectbestanden worden niet mee geüpload.

1. Log in met `npx wrangler login` (Node.js vereist voor de CLI).
2. Publiceer met `npx wrangler deploy` vanuit de projectmap.

Gebruik voor een eigen domein op Workers de domeininstellingen in Cloudflare; `CNAME` is alleen voor GitHub Pages.

### Lokaal

- Windows: dubbelklik op `start.bat`.
- Node.js: `node index.js` (of `npm start`), open `http://localhost:8080`.
