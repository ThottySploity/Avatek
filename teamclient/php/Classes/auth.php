<?php

    include_once("database.php");

    class Auth {
        private $db;

        public function __construct() {
            $this->db = new Database;
        }
        
        public function checkUserExist($username) {
            $this->db->query("SELECT COUNT(*) FROM users WHERE username = :user");
            $this->db->bind(":user", $username, PDO::PARAM_STR);
            $this->db->execute();

            $count = $this->db->single();
            foreach($count as $entry) {
                if($entry > 0) {
                    return true;
                } else {
                    return false;
                }
            }
        }

        public function generatePassword($password) {
            return password_hash($password, PASSWORD_BCRYPT);
        }

        public function checkLogon($username, $password) {
            if($this->checkUserExist($username)) {
                $this->db->query("SELECT password from users WHERE username = :user");
                $this->db->bind(":user", $username, PDO::PARAM_STR);
                $this->db->execute();

                if (password_verify($password, $this->db->single()->password)) {
                    return true;
                }
            }
            return false;
        }
    }