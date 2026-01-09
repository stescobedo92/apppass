# Implementación de UI Interactiva con Ratatui para AppPass

## 📋 Descripción General

Se ha implementado una interfaz de usuario interactiva (TUI - Terminal User Interface) completa para AppPass utilizando Ratatui, que permite realizar todas las operaciones CRUD (Create, Read, Update, Delete) de manera visual e interactiva desde la terminal.

## 🎯 Pasos de Implementación Detallados

### Paso 1: Agregar Dependencias
Se agregaron las siguientes dependencias al `Cargo.toml`:
- **ratatui (v0.29.0)**: Framework para crear interfaces de usuario en terminal
- **crossterm (v0.28.1)**: Manejo de eventos de teclado y control del terminal

```toml
ratatui = "0.29.0"
crossterm = "0.28.1"
```

### Paso 2: Estructura del Módulo UI
Se creó una estructura modular en `src/ui/` con los siguientes componentes:

#### 2.1 `mod.rs` - Módulo Principal
- Función `run_tui()`: Inicializa el terminal en modo raw
- Configura el backend de CrossTerm
- Ejecuta el bucle principal de la aplicación
- Restaura el terminal al salir

#### 2.2 `event.rs` - Manejo de Eventos
- Estructura `EventHandler`: Gestiona eventos del terminal
- Enum `Event`: Define tipos de eventos (Key, Mouse, Resize)
- Polling con timeout configurable para eventos de teclado

#### 2.3 `app.rs` - Estado de la Aplicación
Define la lógica central de la aplicación:

**Modos de Operación:**
- `Menu`: Menú principal
- `Create`: Crear nueva contraseña
- `List`: Listar todas las contraseñas
- `View`: Ver detalles de una contraseña
- `Update`: Actualizar contraseña existente
- `Delete`: Eliminar contraseña

**Características Principales:**
- `InputField`: Estructura para campos de entrada con cursor
- `PasswordEntry`: Representa una entrada de contraseña
- `App`: Estado principal con todos los datos de la aplicación
- Manejo de teclas específico para cada modo

#### 2.4 `ui_render.rs` - Renderizado Visual
Funciones de renderizado para cada pantalla:
- `render_header()`: Encabezado con título de la aplicación
- `render_footer()`: Pie con ayuda contextual
- `render_menu()`: Menú principal con opciones
- `render_create()`: Formulario para crear contraseñas
- `render_list()`: Lista de todas las contraseñas
- `render_view()`: Vista detallada de una contraseña
- `render_update()`: Formulario para actualizar
- `render_delete()`: Formulario para eliminar

### Paso 3: Integración con Main
Se modificó `src/main.rs` para:
1. Importar el módulo `ui`
2. Agregar flag `--ui` al CLI
3. Iniciar la interfaz interactiva cuando se use `--ui`

### Paso 4: Exposición de Constantes
Se modificó `src/app/mod.rs` para hacer públicas las constantes:
- `APP_SERVICE`
- `APP_INDEX`

Estas son necesarias para que el módulo UI pueda acceder al keyring.

## 🎨 Características de la UI

### Pantalla Principal (Menú)
```
┌─────────────────────────────────────────┐
│  🔒 AppPass - Interactive Password Manager  │
├─────────────────────────────────────────┤
│ ▶ Create New Password                  │
│   List All Passwords                   │
│   Update Password                      │
│   Delete Password                      │
│   Exit                                 │
└─────────────────────────────────────────┘
```

**Controles:**
- `↑/↓`: Navegar entre opciones
- `Enter`: Seleccionar opción
- `q/Esc`: Salir

### Crear Contraseña (Create)
- Campo para nombre de aplicación
- Campo opcional para longitud de contraseña
- `Tab`: Cambiar entre campos
- `Enter`: Crear contraseña
- `Esc`: Volver al menú

### Listar Contraseñas (List)
- Muestra todas las contraseñas almacenadas
- Contraseñas enmascaradas con asteriscos
- `↑/↓`: Navegar por la lista
- `Enter`: Ver detalles
- `r`: Refrescar lista
- `Esc`: Volver al menú

### Ver Contraseña (View)
- Muestra nombre de aplicación
- Muestra contraseña completa
- `Enter/Esc`: Volver a la lista

### Actualizar Contraseña (Update)
- Campo para nombre de aplicación
- Campo para nueva contraseña
- `Tab`: Cambiar entre campos
- `Enter`: Actualizar
- `Esc`: Volver al menú

### Eliminar Contraseña (Delete)
- Campo para nombre de aplicación
- `Enter`: Confirmar eliminación
- `Esc`: Cancelar y volver al menú

## 🎯 Funcionalidad CRUD Completa

### Create (Crear)
- Genera contraseñas seguras automáticamente
- Permite especificar longitud personalizada
- Valida que el nombre no esté vacío
- Muestra mensajes de éxito/error

### Read (Leer)
- Lista todas las contraseñas almacenadas
- Permite ver detalles individuales
- Refresco manual de la lista
- Navegación con teclado

### Update (Actualizar)
- Actualiza contraseñas existentes
- Permite establecer contraseñas personalizadas
- Valida campos antes de actualizar

### Delete (Eliminar)
- Elimina contraseñas del keyring
- Confirmación visual del resultado
- Limpia el formulario después de eliminar

## 🔧 Tecnologías Utilizadas

### Ratatui
- Framework moderno para TUIs en Rust
- Renderizado eficiente basado en widgets
- Soporte para layouts flexibles
- Estilos y colores personalizables

### Crossterm
- Manejo multiplataforma del terminal
- Eventos de teclado y mouse
- Control del cursor
- Modo raw del terminal

### Keyring
- Almacenamiento seguro de contraseñas
- Integración con el sistema operativo
- Acceso mediante Entry API

## 🚀 Uso

### Modo CLI (Existente)
```bash
# Crear contraseña
apppass --app gmail

# Listar contraseñas
apppass --list

# Ver contraseña
apppass --get gmail
```

### Modo UI Interactivo (Nuevo)
```bash
# Lanzar interfaz interactiva
apppass --ui
```

## 🎨 Paleta de Colores

- **Cyan**: Título principal, resaltado de selección
- **Yellow**: Campos activos, indicador de edición
- **Green**: Mensajes de éxito
- **Red**: Mensajes de error
- **White**: Texto normal
- **Gray**: Ayuda y texto secundario
- **Black**: Texto sobre fondo de selección

## ✨ Mejoras Futuras Posibles

1. **Búsqueda**: Filtrado en tiempo real de la lista
2. **Ordenamiento**: Opciones para ordenar por nombre o fecha
3. **Generador Avanzado**: Configuración visual del generador
4. **Exportar/Importar**: Interfaz para CSV
5. **Temas**: Soporte para temas de color personalizados
6. **Portapapeles**: Copiar contraseñas al portapapeles
7. **Temporizador**: Auto-clear del portapapeles
8. **Confirmaciones**: Diálogos de confirmación para eliminar

## 📝 Notas de Implementación

- La UI es completamente funcional y no requiere dependencias gráficas
- Funciona en cualquier terminal moderno (Linux, macOS, Windows)
- El estado se mantiene en memoria durante la sesión
- Las contraseñas se almacenan de forma segura en el keyring del sistema
- La navegación es intuitiva siguiendo convenciones estándar de TUIs

## 🔐 Seguridad

- Las contraseñas se muestran solo cuando el usuario lo solicita
- En la lista, las contraseñas están enmascaradas
- No se almacenan contraseñas en texto plano en archivos
- Integración con el keyring del sistema operativo
