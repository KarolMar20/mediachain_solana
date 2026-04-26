use anchor_lang::prelude::*;

// ID del programa (Se genera al hacer build en SolPG)
declare_id!("GE9v9pDssAaU5oCkqKTVDMAhgiJ42E9p9V4DRGsktY5S");

#[program]
pub mod mediachain_solana {
    use super::*;

    // 1. CREATE (PDA): Inicializa la bitácora personal de medios
    pub fn inicializar_bitacora(ctx: Context<CrearBitacora>, nombre_usuario: String) -> Result<()> {
        let bitacora = &mut ctx.accounts.bitacora;
        bitacora.owner = ctx.accounts.owner.key();
        bitacora.nombre_usuario = nombre_usuario;
        bitacora.resenas = Vec::new();
        
        msg!("Bitácora de medios de '{}' creada.", bitacora.nombre_usuario);
        Ok(())
    }

    // 2. CREATE (Dato): Registra una reseña exigiendo todos los parámetros manuales
    pub fn agregar_resena(
        ctx: Context<GestionarBitacora>, 
        titulo: String, 
        calificacion: u8, 
        visto_completo: bool
    ) -> Result<()> {
        let bitacora = &mut ctx.accounts.bitacora;
        require!(bitacora.owner == ctx.accounts.owner.key(), Errores::AccesoDenegado);

        let nueva_resena = Resena {
            titulo,
            calificacion,
            visto_completo,
        };

        bitacora.resenas.push(nueva_resena);
        msg!("Reseña agregada a la blockchain.");
        Ok(())
    }

    // 3. UPDATE: Modifica una reseña existente (Búsqueda por título)
    pub fn editar_resena(
        ctx: Context<GestionarBitacora>, 
        titulo: String, 
        nueva_calificacion: u8, 
        nuevo_visto_completo: bool
    ) -> Result<()> {
        let bitacora = &mut ctx.accounts.bitacora;
        require!(bitacora.owner == ctx.accounts.owner.key(), Errores::AccesoDenegado);

        let lista = &mut bitacora.resenas;
        for i in 0..lista.len() {
            if lista[i].titulo == titulo {
                lista[i].calificacion = nueva_calificacion;
                lista[i].visto_completo = nuevo_visto_completo;
                msg!("Reseña de '{}' actualizada.", titulo);
                return Ok(());
            }
        }
        Err(Errores::ResenaNoEncontrada.into())
    }

    // 4. DELETE: Elimina una reseña del historial
    pub fn eliminar_resena(ctx: Context<GestionarBitacora>, titulo: String) -> Result<()> {
        let bitacora = &mut ctx.accounts.bitacora;
        require!(bitacora.owner == ctx.accounts.owner.key(), Errores::AccesoDenegado);

        let lista = &mut bitacora.resenas;
        let index = lista.iter().position(|r| r.titulo == titulo);

        if let Some(i) = index {
            lista.remove(i);
            msg!("Reseña de '{}' eliminada.", titulo);
            Ok(())
        } else {
            Err(Errores::ResenaNoEncontrada.into())
        }
    }

    // 5. READ: Visualiza todo el historial de reseñas
    pub fn ver_bitacora(ctx: Context<GestionarBitacora>) -> Result<()> {
        msg!("Usuario: {}", ctx.accounts.bitacora.nombre_usuario);
        msg!("Historial de Reseñas: {:#?}", ctx.accounts.bitacora.resenas);
        Ok(())
    }
}

// --- ESTADO DEL PROGRAMA ---

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Resena {
    #[max_len(40)]
    pub titulo: String,
    pub calificacion: u8,
    pub visto_completo: bool,
}

#[account]
#[derive(InitSpace)]
pub struct BitacoraMedios {
    pub owner: Pubkey,
    #[max_len(40)]
    pub nombre_usuario: String,
    #[max_len(20)] // Capacidad para 20 reseñas
    pub resenas: Vec<Resena>,
}

// --- CONTEXTOS ---

#[derive(Accounts)]
pub struct CrearBitacora<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = 8 + BitacoraMedios::INIT_SPACE,
        seeds = [b"mediachain", owner.key().as_ref()],
        bump
    )]
    pub bitacora: Account<'info, BitacoraMedios>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GestionarBitacora<'info> {
    pub owner: Signer<'info>,
    #[account(mut)]
    pub bitacora: Account<'info, BitacoraMedios>,
}

// --- ERRORES ---

#[error_code]
pub enum Errores {
    #[msg("No tienes permisos para editar esta bitácora.")]
    AccesoDenegado,
    #[msg("La reseña buscada no existe en el registro.")]
    ResenaNoEncontrada,
}
