use std::{net::Ipv6Addr, str::FromStr, time::Duration};

use anyhow::{Error, Result};
use icmp_socket::{packet::WithEchoRequest, IcmpSocket, IcmpSocket6, Icmpv6Packet};
use image::Rgb;
use tracing::{info, trace};

/// origin: fdcf:8538:9ad5:3333
/// clearnet: 2a09:b280:ff82:4242
pub static mut PREFIX: String = String::new();

pub fn new_socket() -> Result<IcmpSocket6> {
    let mut socket = IcmpSocket6::new()?;
    socket
        .bind(Ipv6Addr::from_str("::0")?)
        .map_err(|e| Error::from(e))?;
    Ok(socket)
}

pub static mut QPS_COUNTER: usize = 0;

pub async fn draw_pixel(
    socket: &mut IcmpSocket6,
    x: u16,
    y: u16,
    color: &Rgb<u8>,
    fast: bool,
) -> Result<()> {
    //assert!(x < 512, "x = {} >= 512", x);
    //assert!(y < 512, "y = {} >= 512", y);
    if x >= 512 || y >= 512 {
        return Ok(());
    }

    let r = color.0[0];
    let g = color.0[1];
    let b = color.0[2];
    let addr_str = format!("{}:{x:0>4x}:{y:0>4x}:11{r:0>2x}:{g:0>2x}{b:0>2x}", unsafe {
        &PREFIX
    });
    let addr = Ipv6Addr::from_str(&addr_str)?;

    let mut succ = 0;
    loop {
        if unsafe { QPS_COUNTER } > 1024 {
            trace!("QPS overload");
            tokio::time::sleep(Duration::from_millis(10)).await;
            continue;
        }
        let pkt = Icmpv6Packet::with_echo_request(4431, x + y, vec![])
            .map_err(|e| Error::msg(e.to_string()))?;
        if let Err(err) = socket.send_to(addr, pkt) {
            if let Some(code) = err.raw_os_error() {
                if code == 105 {
                    trace!("os buffer is full, delaying");
                    tokio::time::sleep(Duration::from_millis(10)).await;
                    continue;
                }
            }
            return Err(err.into());
        } else {
            unsafe {
                QPS_COUNTER += 1;
            }
            succ += 1;
        }
        if succ >= 3 || fast {
            break;
        }
    }

    Ok(())
}

pub async fn fill_rect(
    socket: &mut IcmpSocket6,
    x: u16,
    y: u16,
    w: u16,
    h: u16,
    color: &Rgb<u8>,
) -> Result<()> {
    info!(x, y, w, h, "drawing rect");
    for x in x..x + w {
        for y in y..y + h {
            draw_pixel(socket, x, y, color, true).await?;
        }
        if x >= 512 {
            break;
        }
    }
    Ok(())
}
