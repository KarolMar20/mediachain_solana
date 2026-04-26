# 🎬 MediaChain Solana


Sistema de bitácora de medios desarrollado como **Solana Program** utilizando **Rust** y el framework **Anchor**.  

Este proyecto implementa un sistema **CRUD** para registrar y gestionar reseñas de contenido (películas, series, anime, etc.) directamente en blockchain, aplicando:

- 🔑 Program Derived Addresses (PDAs)  
- ⚡ Optimización de memoria *On-Chain*  
- 🔒 Seguridad basada en firmas  

---

## 📚 Descripción

**MediaChain Solana** simula una bitácora personal donde cada usuario puede:

- Crear su historial de medios  
- Registrar reseñas de contenido  
- Editar calificaciones y estado (visto/no visto)  
- Eliminar reseñas  
- Consultar su historial en blockchain  

---

## 🧠 Arquitectura y Estructuras de Datos

En Solana es necesario definir el tamaño de los datos para calcular correctamente la renta (*rent*).

### 📦 PDA Principal: `BitacoraMedios`

Cuenta raíz que almacena todas las reseñas del usuario.

```rust
#[account]
#[derive(InitSpace)]
pub struct BitacoraMedios {
    pub owner: Pubkey,
    #[max_len(40)]
    pub nombre_usuario: String,
    #[max_len(20)]
    pub resenas: Vec<Resena>,
}
```

---

### 🧩 Estructura Interna: `Resena`

Cada reseña contiene:

- `titulo (String)` → nombre del contenido  
- `calificacion (u8)` → puntuación (ej. 1–10)  
- `visto_completo (bool)` → indica si se terminó el contenido  

```rust
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Resena {
    #[max_len(40)]
    pub titulo: String,
    pub calificacion: u8,
    pub visto_completo: bool,
}
```

---

## 🔒 Seguridad

El contrato asegura que solo el propietario pueda modificar su bitácora:

```rust
require!(
    bitacora.owner == ctx.accounts.owner.key(),
    Errores::AccesoDenegado
);
```

✔ Protege la información del usuario  
✔ Evita modificaciones por terceros  

---

## ⚙️ Funcionalidad (CRUD)

### 🟢 Inicializar Bitácora

Crea la cuenta principal usando:

```rust
[b"mediachain", owner.key().as_ref()]
```

Inicializa:
- Owner  
- Nombre del usuario  
- Lista vacía de reseñas  

---

### ➕ Agregar Reseña

- Recibe:
  - título  
  - calificación  
  - estado (visto completo)  
- Inserta en el vector con `.push()`  

---

### ✏️ Editar Reseña

- Busca por `titulo`  
- Actualiza:
  - calificación  
  - estado (`visto_completo`)  

---

### ❌ Eliminar Reseña

```rust
.iter().position(|r| r.titulo == titulo)
```

- Si existe → `.remove(index)`  
- Si no → error `ResenaNoEncontrada`  

---

### 📖 Ver Bitácora

```rust
msg!("Historial de Reseñas: {:#?}", bitacora.resenas);
```

Muestra todas las reseñas en logs *On-Chain*

---

## 🧪 Despliegue en Solana Playground

1. Copia el código en `lib.rs`  
2. Ejecuta:

```bash
cargo clean
```

3. Haz clic en **Build**  
4. Haz clic en **Deploy (Devnet)**  

---

## 🧑‍💻 Pruebas

Puedes interactuar con el contrato usando:

- Pestaña **Test** del Playground  
- Scripts en TypeScript:

```ts
pg.program.methods...
```

Parámetros:
- `titulo: String`  
- `calificacion: u8`  
- `visto_completo: bool`  

---

## ⚠️ Manejo de Errores

```rust
#[error_code]
pub enum Errores {
    #[msg("No tienes permisos para editar esta bitácora.")]
    AccesoDenegado,
    #[msg("La reseña buscada no existe en el registro.")]
    ResenaNoEncontrada,
}
```

---

## 📌 Conclusión

Este proyecto demuestra:

- Gestión de datos personales en blockchain  
- Seguridad mediante validación de firmas  
- Uso eficiente de estructuras dinámicas  
- Implementación de CRUD en un caso práctico (Media Tracker)  

---

## 🚀 Próximos pasos

- Añadir categorías (película, serie, anime)  
- Implementar promedio automático de calificaciones  
- Integrar frontend tipo Letterboxd  
- Compartir reseñas públicamente  

---
