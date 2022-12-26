import fs from 'fs'

fs.writeFileSync('test.bin', Buffer.alloc(0xFFFF, 0))