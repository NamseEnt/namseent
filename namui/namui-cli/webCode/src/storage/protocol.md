# ProtocolBuffer

All value are 4 bytes for simplicity.

Use first byte to wait/notify request or response.
When the request written, the first 4 byte will be the request type.
When the response written, the first 4 byte will be 0.

# Requests and Responses

- open-read(1)
  - description: open file for read. This locks key in shared mode.
  - request
    - key ptr
    - key len
  - response
    - fd: 0 if file not exists
- read(2)
  - description: put data to the buffer. Automatically unlock on EOF.
  - request
    - fd
    - buffer ptr
    - buffer len
  - response
    - byte length copied to the buffer.
    - 0 if not done yet, 1 if EOF
- open-write(3)
  - description: open file for write. This locks key in exclusive mode. create file if not exists.
  - request
    - key ptr
    - key len
  - response
    - fd
- write(4)
  - description: write data to the file.
  - request
    - fd
    - buffer ptr
    - buffer len
  - response
    - return code.
      - 0x00: success
      - 0x01: out of space(OPFS QuotaExceededError)
- flush(5)
  - description: flush
  - request
    - fd
  - response
- close(6)
  - description: close file. This unlocks key.
  - request
    - fd
  - response
    - none
- delete(7)
  - description: delete file.
  - request
    - key ptr
    - key len
  - response
    - none
