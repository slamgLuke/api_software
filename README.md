# Examen Final Software

## Integrantes
- Lucas Carranza Bueno (202210073)
- Adrian Céspedes Zevallos (202210088)


## Ejecución
Para ejecutar el programa, se debe ejecutar el siguiente comando:
```bash
cargo run
```
Esto abre el endpoint en el puerto 3000.


## Pruebas Unitarias
Para ejecutar las pruebas unitarias, se debe ejecutar el siguiente comando:
```bash
cargo test
```


## Pregunta 3
Se requiere realizar un cambio en el software para que soporte un valor máximo de 200 soles a transferir por día.
1. ¿Qué cambiaría en el código? (Clases / Métodos) - No realizar la implementación, sólo descripción.
- Se debe agregar la constante MAX_DAILY_TRANSFER = 200
- Agregar una función get_daily_transfer_amount(user) que devuelva el monto transferido en el día por el usuario. Para esto, se debe agregar un campo daily_transfer_amount en la estructura User. Este campo se debe actualizar cada vez que se realiza una transferencia, y se debe reiniciar a 0 cada vez que cambia el día
- Antes de realizar una transferencia, se debe verificar que el monto a transferir + get_daily_transfer_amount(user) <= MAX_DAILY_TRANSFER. Esto último se puede encapsular en una función can_transfer_amount(user, amount) que devuelva un booleano
- En la función transfer_amount, se debe llamar a can_transfer_amount(user, amount) antes de realizar la transferencia
- Finalmente, negar la transferencia si can_transfer_amount(user, amount) es falso, y permitir la transferencia si es verdadero

2. ¿Qué casos de prueba nuevos serían necesarios?
- Caso de éxito: Transferir exactamente 200 soles en un día
- Caso de éxito: Transferir 200 soles en un día, y 200 soles al día siguiente
- Caso de éxito: Transferir 100 soles en un día, luego 70 soles y finalmente 30 soles

- Caso de error: Transferir 201 soles en un día
- Caso de error: Transferir 100 soles en un día, luego 70 soles y finalmente 31 soles

3. ¿Los casos de prueba existentes garantizan que no se introduzcan errores en la funcionalidad existente?
- Se abarcan errores comunes como id de usuario inválido, monto mayor al saldo, monto inválido.
- Se abarcan los casos de éxito para transferencias válidas, acceso a historial de un usuario válido.
- Además los casos de prueba se realizan no de manera interna, sino haciendo requests internamente al servidor. Esto hace que los endpoints se prueben más fielmente a como se usarían en producción.
- Si bien pueden haber más casos de error, los casos de prueba existentes garantizan que no se introduzcan errores en la funcionalidad existente, ya que se prueban los casos más comunes y críticos.
