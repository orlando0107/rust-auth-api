# 📚 Documentación de la API

## 🔍 Acceso a la Documentación

La documentación de la API está disponible a través de Swagger UI en:

```shell
http://localhost:3000/swagger-ui/
```

También puedes acceder al archivo OpenAPI directamente en:

```shell
http://localhost:3000/api-docs/openapi.json
```

## 📋 Endpoints Disponibles

### 🔐 Autenticación

#### 1. Registro de Usuario

- **Método**: `POST`
- **Ruta**: `/auth/register`
- **Descripción**: Crea un nuevo usuario en el sistema
- **Cuerpo de la Solicitud**:

```json
{
    "email": "usuario@ejemplo.com",
    "password": "contraseña123",
    "name": "Nombre Usuario"
}

```

- **Respuesta Exitosa** (201 Created):

```json
{
    "id": 1,
    "email": "usuario@ejemplo.com",
    "name": "Nombre Usuario"
}
```

#### 2. Inicio de Sesión

- **Método**: `POST`
- **Ruta**: `/auth/login`
- **Descripción**: Autentica a un usuario y genera un token JWT
- **Cuerpo de la Solicitud**:

```json
{
    "email": "usuario@ejemplo.com",
    "password": "contraseña123"
}

```

- **Respuesta Exitosa** (200 OK):

```json
{
    "session_id": "uuid-de-sesion",
    "token": "jwt-token",
    "user": {
        "id": 1,
        "email": "usuario@ejemplo.com",
        "name": "Nombre Usuario"
    }
}
```

#### 3. Cierre de Sesión

- **Método**: `POST`
- **Ruta**: `/auth/logout`
- **Descripción**: Cierra la sesión del usuario actual
- **Headers Requeridos**:
  - `Authorization: Bearer <jwt-token>`
- **Respuesta Exitosa** (200 OK):

```json
{
    "message": "Successfully logged out"
}
```

### 👤 Perfil de Usuario

#### 1. Obtener Perfil

- **Método**: `GET`
- **Ruta**: `/profile`
- **Descripción**: Obtiene la información del perfil del usuario autenticado
- **Headers Requeridos**:
  - `Authorization: Bearer <jwt-token>`
- **Respuesta Exitosa** (200 OK):

```json
{
    "id": 1,
    "email": "usuario@ejemplo.com",
    "name": "Nombre Usuario"
}
```

## ⚠️ Códigos de Error

### 400 Bad Request

- **Causa**: Datos de entrada inválidos
- **Ejemplo**:

```json
{
    "errors": {
        "email": ["debe ser un email válido"],
        "password": ["debe tener al menos 8 caracteres"]
    }
}
```

### 401 Unauthorized

- **Causa**: Token inválido o sesión expirada
- **Ejemplo**:

```json
{
    "error": "No active session"
}
```

### 500 Internal Server Error

- **Causa**: Error interno del servidor
- **Ejemplo**:

```json
{
    "error": "Internal server error"
}
```

## 🔐 Seguridad

### Autenticación

- Todos los endpoints protegidos requieren un token JWT válido
- El token debe incluirse en el header `Authorization` con el formato `Bearer <token>`
- Los tokens tienen una validez de 1 hora

### Validación

- Se valida el formato del email
- Las contraseñas deben tener al menos 8 caracteres
- Los nombres no pueden estar vacíos

## 🚀 Ejemplos de Uso

### Registro de Usuario

```bash
curl -X POST http://localhost:3000/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email":"usuario@ejemplo.com","password":"contraseña123","name":"Nombre Usuario"}'
```

### Inicio de Sesión

```bash
curl -X POST http://localhost:3000/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"usuario@ejemplo.com","password":"contraseña123"}'
```

### Obtener Perfil

```bash
curl http://localhost:3000/profile \
  -H "Authorization: Bearer <jwt-token>"
```

### Cierre de Sesión

```bash
curl -X POST http://localhost:3000/auth/logout \
  -H "Authorization: Bearer <jwt-token>"
```

## 📝 Notas Adicionales

1. **Tokens JWT**:
   - Los tokens incluyen el ID del usuario y la fecha de expiración
   - Se almacenan en Redis para validación de sesión
   - Se invalidan al cerrar sesión

2. **Sesiones**:
   - Cada sesión tiene un ID único (UUID)
   - Las sesiones expiran después de 1 hora
   - Se pueden cerrar manualmente con el endpoint de logout

3. **Seguridad**:
   - Las contraseñas se almacenan con hash bcrypt
   - Los tokens JWT se firman con una clave secreta
   - Las sesiones se validan contra Redis

4. **Rendimiento**:
   - Se utiliza connection pooling para PostgreSQL
   - Redis actúa como caché para las sesiones
   - Las validaciones son eficientes y asíncronas

## 🛠️ Requisitos del Sistema

### Dependencias

- Rust 1.70 o superior
- PostgreSQL 15 o superior
- Redis 7 o superior
- Docker y Docker Compose (opcional)

### Variables de Entorno

```env
DATABASE_URL=postgres://usuario:contraseña@localhost:5432/basedatos
REDIS_URL=redis://localhost:6380
JWT_SECRET=tu-clave-secreta-aqui
```

## 🔄 Flujo de Desarrollo

### 1. Configuración Inicial

```bash
# Clonar el repositorio
git clone https://github.com/orlandocardenas/rust-auth-api.git
cd rust-auth-api

# Configurar variables de entorno
cp .env.example .env
# Editar .env con tus credenciales

# Iniciar servicios con Docker
docker-compose up -d
```

### 2. Desarrollo

```bash
# Compilar y ejecutar en modo desarrollo
cargo run

# Compilar y ejecutar con logs detallados
RUST_LOG=debug cargo run

# Ejecutar pruebas
cargo test
```

### 3. Despliegue

```bash
# Compilar para producción
cargo build --release

# Ejecutar en producción
./target/release/backend-rust
```

## 📊 Monitoreo y Logs

### Niveles de Log

- `error`: Errores críticos
- `warn`: Advertencias
- `info`: Información general
- `debug`: Información detallada para desarrollo
- `trace`: Información muy detallada

### Ejemplo de Configuración de Logs

```rust
RUST_LOG=info,backend_rust=debug cargo run
```

## 🔍 Troubleshooting

### Problemas Comunes

1. **Error de Conexión a PostgreSQL**
   - Verificar que PostgreSQL esté ejecutándose
   - Comprobar las credenciales en `.env`
   - Verificar que la base de datos exista

2. **Error de Conexión a Redis**
   - Verificar que Redis esté ejecutándose
   - Comprobar el puerto en `.env`
   - Verificar que Redis acepte conexiones

3. **Errores de Autenticación**
   - Verificar el formato del token JWT
   - Comprobar que la sesión exista en Redis
   - Verificar la expiración del token

### Soluciones

1. **Reiniciar Servicios**

```bash
docker-compose restart
```

2.**Verificar Logs**

```bash
docker-compose logs -f
```

3.**Limpiar Sesiones**

```bash
redis-cli -p 6380 FLUSHALL
```

## 📈 Mejores Prácticas

### Seguridad

1. **Tokens JWT**
   - Nunca compartir la clave secreta
   - Rotar tokens regularmente
   - Implementar revocación de tokens

2. **Contraseñas**
   - Usar contraseñas fuertes
   - Implementar política de expiración
   - No almacenar contraseñas en texto plano

3. **Sesiones**
   - Limitar número de sesiones activas
   - Implementar timeout automático
   - Registrar intentos fallidos

### Rendimiento

1. **Base de Datos**
   - Usar índices apropiados
   - Optimizar consultas frecuentes
   - Implementar caché cuando sea posible

2. **Redis**
   - Configurar memoria adecuada
   - Implementar persistencia
   - Monitorear uso de memoria

## 🤝 Contribución

### Guía de Contribución

1. Fork el repositorio
2. Crear una rama para tu feature
3. Hacer commit de tus cambios
4. Push a la rama
5. Crear un Pull Request

### Estándares de Código

- Seguir las convenciones de Rust
- Documentar código público
- Escribir pruebas unitarias
- Mantener el código limpio y organizado

## 📚 Recursos Adicionales

### Documentación Oficial

- [Rust](https://www.rust-lang.org/learn)
- [Actix Web](https://actix.rs/docs/)
- [SQLx](https://github.com/launchbadge/sqlx)
- [Redis](https://redis.io/documentation)

### Tutoriales

- [Rust para Principiantes](https://doc.rust-lang.org/book/)
- [Autenticación con JWT](https://jwt.io/introduction)
- [Redis para Desarrolladores](https://redis.io/topics/data-types)

### Comunidad

- [Rust Users Forum](https://users.rust-lang.org/)
- [Rust Discord](https://discord.gg/rust-lang)
- [Stack Overflow Rust](https://stackoverflow.com/questions/tagged/rust)

## 📞 Soporte

### Canales de Soporte

- [Issues en GitHub](https://github.com/orlandocardenas/rust-auth-api/issues)
- [Discord del Proyecto](https://discord.gg/tu-invitacion)
- [Email de Soporte](mailto:soporte@ejemplo.com)

### Política de Soporte

- Respuesta en 24 horas hábiles
- Prioridad a issues críticos
- Documentación actualizada regularmente
