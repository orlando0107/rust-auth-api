# 🦀 Rust Auth API

![Rust Logo](https://www.rust-lang.org/static/images/rust-social-wide.jpg)

## 📝 Descripción

Rust Auth API - Sistema de autenticación seguro y escalable en Rust con PostgreSQL y Redis. Incluye registro, login, logout y gestión de perfiles con JWT y sesiones. El proyecto está diseñado para ser un ejemplo práctico de cómo construir una API REST segura y escalable con Rust.

## 🎯 Características

- Sistema de autenticación completo (registro, login, logout)
- Almacenamiento seguro de contraseñas con bcrypt
- Tokens JWT para autenticación
- Sesiones manejadas con Redis
- Documentación con Swagger/OpenAPI
- Validación de datos de entrada
- Manejo de errores robusto
- Logging detallado

## 🏗️ Estructura del Proyecto

```shell
backend-rust/
├── src/
│   ├── config/          # Configuración de la aplicación
│   │   ├── database.rs  # Configuración de PostgreSQL
│   │   └── redis.rs     # Configuración de Redis
│   ├── handlers/        # Manejadores de rutas
│   │   ├── auth.rs      # Endpoints de autenticación
│   │   └── profile.rs   # Endpoints de perfil
│   ├── middleware/      # Middleware de la aplicación
│   │   └── auth.rs      # Middleware de autenticación
│   ├── models/          # Modelos de datos
│   │   └── user.rs      # Modelo de usuario
│   ├── services/        # Servicios de la aplicación
│   │   └── auth.rs      # Servicios de autenticación
│   └── main.rs          # Punto de entrada
├── .env                 # Variables de entorno
├── .env.example         # Ejemplo de variables de entorno
├── Cargo.toml           # Dependencias y configuración
└── docker-compose.yml   # Configuración de Docker
```

## 🛠️ Módulos Utilizados

### Dependencias Principales

- **actix-web**: Framework web para Rust
- **sqlx**: ORM asíncrono para PostgreSQL
- **redis**: Cliente Redis para Rust
- **jsonwebtoken**: Manejo de tokens JWT
- **bcrypt**: Encriptación de contraseñas
- **validator**: Validación de datos
- **utoipa**: Documentación OpenAPI
- **env_logger**: Logging

### Créditos a los Creadores

- [Actix Web](https://github.com/actix/actix-web) - Framework web
- [SQLx](https://github.com/launchbadge/sqlx) - ORM asíncrono
- [Redis-rs](https://github.com/redis-rs/redis-rs) - Cliente Redis
- [jsonwebtoken](https://github.com/Keats/jsonwebtoken) - Manejo de JWT
- [bcrypt-rs](https://github.com/Keats/rust-bcrypt) - Encriptación
- [validator](https://github.com/Keats/validator) - Validación
- [utoipa](https://github.com/juhaku/utoipa) - Documentación OpenAPI

## 🐳 Docker

El proyecto utiliza Docker para facilitar el desarrollo y despliegue:

```yaml
version: '3.8'

services:
  postgres:
    image: postgres:latest
    environment:
      POSTGRES_USER: ${POSTGRES_USER}
      POSTGRES_PASSWORD: ${POSTGRES_PASSWORD}
      POSTGRES_DB: ${POSTGRES_DB}
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data

  redis:
    image: redis:latest
    ports:
      - "6380:6379"
    volumes:
      - redis_data:/data

volumes:
  postgres_data:
  redis_data:
```

## 🔄 Flujo de Autenticación

1. **Registro de Usuario**
   - El usuario envía email, contraseña y nombre
   - La contraseña se hashea con bcrypt
   - Se crea el usuario en PostgreSQL

2. **Inicio de Sesión**
   - El usuario envía email y contraseña
   - Se verifica la contraseña con bcrypt
   - Se genera un token JWT
   - Se crea una sesión en Redis
   - Se devuelve el token y la session_id

3. **Acceso a Perfil**
   - El usuario envía el token JWT
   - El middleware valida el token
   - Se verifica la sesión en Redis
   - Se devuelve la información del perfil

4. **Cierre de Sesión**
   - El usuario envía el token JWT
   - Se elimina la sesión de Redis
   - Se invalida el token

## 🚀 Instalación

1.Clonar el repositorio:

```bash
git clone https://github.com/orlando0107/rust-auth-api.git
cd rust-auth-api
```

2.Configurar variables de entorno:

```bash
cp .env.example .env
# Editar .env con tus credenciales
```

3.Iniciar servicios con Docker:

```bash
docker-compose up -d
```

4.Crear la tabla de usuarios:

```sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL
);
```

5.Compilar y ejecutar:

```bash
cargo run
```

## 📚 Documentación

La documentación de la API está disponible en:

- Swagger UI: `http://localhost:3000/swagger-ui/`
- OpenAPI: `http://localhost:3000/api-docs/openapi.json`

## 📋 Endpoints

### Autenticación

- `POST /auth/register`: Registrar usuario
- `POST /auth/login`: Iniciar sesión
- `POST /auth/logout`: Cerrar sesión

### Perfil

- `GET /profile`: Obtener perfil (requiere autenticación)

## 📜 Licencia

Este proyecto está bajo la licencia MIT.

Copyright (c) 2024 Orlando Cardenas

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

## 👥 Contribuciones

Las contribuciones son bienvenidas. Por favor, lee las guías de contribución antes de enviar un pull request.

## 📞 Contacto

Orlando Cardenas - [@orlando0107](https://github.com/orlando0107)
