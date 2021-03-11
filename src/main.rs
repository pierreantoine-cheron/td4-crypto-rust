// import des modules utilises
use openssl::rand::rand_bytes;
use openssl::symm::{decrypt, encrypt, Cipher};
use std::env;
use std::fs;

// longueur de la clef en octets
const LEN: usize = 8;

fn main() {
    chiffrage_dechiffrage_chaine_de_caracteres();
    chiffrage_dechiffrage_fichier();
}

fn chiffrage_dechiffrage_chaine_de_caracteres() {
    println!("DEBUT CHIFFRAGE CHAINE DE CARACTERES");
    // declaration de la clef et du vecteur d'initialisation (iv)
    // meme si l'algorithme DES ne requiert pas d'iv, la methode generique permettant de chiffrer les donnees necessite un iv
    let mut key = [0; LEN];
    let mut iv = [0, 16];

    // generation de la clef et de l'iv
    rand_bytes(&mut key[..]).unwrap();
    rand_bytes(&mut iv[..]).unwrap();
    println!("clef aleatoire: {:?}", key);
    println!("vecteur d'initialisation aleatoire: {:?}", iv);

    // choix de l'algo de chiffrage, je n'ai pas reussi a faire fonctionner la version cbc de DES : "ctrl operation not implemented"
    let cipher = Cipher::des_ecb();

    // donnees arbitraires a chiffrer
    let data = b"mon texte secret";

    // chiffrage des donnees
    let texte_chiffre = encrypt(cipher, &key[..], Some(&iv), data).unwrap();
    println!("donnees chiffrees: {:?}", texte_chiffre);

    // dechiffrage des donnees
    let texte_dechiffre = decrypt(cipher, &key[..], Some(&iv), &texte_chiffre).unwrap();

    println!("{}", String::from_utf8_lossy(&texte_dechiffre[..]));
    println!("FIN CHIFFRAGE CHAINE DE CARACTERES");
}

fn chiffrage_dechiffrage_fichier() {
    println!("DEBUT CHIFFRAGE FICHIER");

    // recuperation des arguments du programme
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        // recuperation de l'argument contenant le chemin du fichier
        let path = &args[1];

        // lecture du fichier et conversion en tableau d'octets
        let data = fs::read_to_string(path).unwrap();
        let data = data.as_bytes();
        
        // declaration de la clef et du vecteur d'initialisation (iv)
        // meme si l'algorithme DES ne requiert pas d'iv, la methode generique permettant de chiffrer les donnees necessite un iv
        let mut key = [0; LEN];
        let mut iv = [0, 16];

        // generation de la clef et de l'iv
        rand_bytes(&mut key[..]).unwrap();
        rand_bytes(&mut iv[..]).unwrap();
        println!("clef aleatoire: {:?}", key);
        println!("vecteur d'initialisation aleatoire: {:?}", iv);

        // choix de l'algo de chiffrage, je n'ai pas reussi a faire fonctionner la version cbc de DES : "ctrl operation not implemented"
        let cipher = Cipher::des_ecb();

        // chiffrage des donnees
        let texte_chiffre = encrypt(cipher, &key[..], Some(&iv), data).unwrap();
        println!("donnees chiffrees: {:?}", texte_chiffre);

        // dechiffrage des donnees
        let texte_dechiffre = decrypt(cipher, &key[..], Some(&iv), &texte_chiffre).unwrap();

        println!("{}", String::from_utf8_lossy(&texte_dechiffre[..]));
    } else {
        println!("nombre d'arguments incorrect, veuillez donner le chemin complet du fichier que vous voulez chiffrer en unique argument du programme");
    }

    println!("FIN CHIFFRAGE FICHIER");
}
