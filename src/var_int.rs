use tokio::{net::TcpStream, io::{AsyncReadExt, AsyncWriteExt}};

#[cfg(not(no_std))]
type Result<T> = std::io::Result<T>;

async fn write_varint_i32(stream: &mut TcpStream, mut value: i32) -> Result<usize> {
    let mut buffer = [0];
    let mut cnt = 0;

    while value != 0 {
        buffer[0] = (value & 0b0111_1111) as u8;
        value = (value >> 7) & (i32::max_value() >> 6);
        if value != 0 {
            buffer[0] |= 0b1000_0000;
        }

        cnt += stream.write(&mut buffer).await.unwrap();
    }

    return Ok(cnt)
}

async fn write_varint_i64(stream: &mut TcpStream, mut value: i64) -> Result<usize> {
    let mut buffer = [0];
    let mut cnt = 0;

    while value != 0 {
        buffer[0] = (value & 0b0111_1111) as u8;
        value = (value >> 7) & (i64::max_value() >> 6);
        if value != 0 {
            buffer[0] |= 0b1000_0000;
        }

        cnt += stream.write(&mut buffer).await.unwrap();
    }

    return Ok(cnt)
}

pub async fn read_varint_i32(stream: &mut TcpStream) -> Result<i32> {
    let mut buffer = [0];
    let mut answer = 0;

    for i in 0..4 {
        stream.read_exact(&mut buffer).await.unwrap();
        answer |= ((buffer[0] & 0b0111_1111) as i32) << 7 * i;
        if buffer[0] & 0b1000_0000 == 0 {
            break;
        }
    }

    return Ok(answer)
} 

pub async fn read_varint_i64(stream: &mut TcpStream) -> Result<i64> {
    let mut buffer = [0];
    let mut answer = 0;

    for i in 0..8 {
        stream.read_exact(&mut buffer).await.unwrap();
        answer |= ((buffer[0] & 0b0111_1111) as i64) << 7 * i;
        if buffer[0] & 0b1000_0000 == 0 {
            break;
        }
    }

    return Ok(answer)
}