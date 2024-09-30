<?php

    class Base64 {
        const BASE64_CHARS = [
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
            'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f',
            'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
            'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '-', '_'
        ];

        public function encode($input){
            $result = '';
            $i = 0;
            $bits = 0;
            $current = 0;

            foreach (str_split($input) as $byte) {
                $current = ($current << 8) | ord($byte);
                $bits += 8;

                while ($bits >= 6) {
                    $bits -= 6;
                    $index = ($current >> $bits) & 0x3F;
                    $result .= self::BASE64_CHARS[$index];
                    $i++;
                }
            }

            if ($bits > 0) {
                $current <<= (6 - $bits);
                $index = $current & 0x3F;
                $result .= self::BASE64_CHARS[$index];
            }

            while ($i % 4 != 0) {
                $result .= '=';
                $i++;
            }

            return $result;
        }

        public function decode($input)
        {
            $result = '';
            $bits = 0;
            $current = 0;

            foreach (str_split($input) as $char) {
                if ($char === '=') {
                    break;
                }

                $index = array_search($char, self::BASE64_CHARS);
                if ($index === false) {
                    throw new Exception("Invalid character in input");
                }

                $current = ($current << 6) | $index;
                $bits += 6;

                while ($bits >= 8) {
                    $bits -= 8;
                    $result .= chr(($current >> $bits) & 0xFF);
                }
            }

            return $result;
        }
    }