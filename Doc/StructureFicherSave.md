# Structure des différents json du projet

## Fichiers récents

```json
{
    "files": [
        {
            "name": string,
            "path": string,
            "pinned": bool,
            "last_opened": date
        }
    ]
}
```

## Fichier de stockage .mmt

Les fichier de stockage en `.mmt` sont simplement un json et un dossier d'image mis ensemble à l'aide de tar et renommé.

Arborescences des fichiers .mmt :

```
structure.json
asset/
  ├ <hash>.webp
  └ <hash>.webp
```

### Champs

- hash :
- row, images, remaining_images : Leurs positions sont défini par leur ordre dans le json.

```json
{
    "name": string,
    "rows": [
        {
            "name": string,
            "color": string, // hexa
            "images": [
                {
                    "hash": string // blake3
                }
            ]
        }
    ],
    "remaining_images": [
        "hash": string
    ]
}
```
