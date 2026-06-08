# C -> Rust Dönüşüm Kuralları (Migration Rules)

Bu doküman, Linux çekirdeğinin C kütüphanelerini Rust diline taşırken uyulması **zorunlu olan** temel kuralları tanımlar.

---

### 📌 Kural 1: Linus Torvalds Tarzı Geliştirme (Linus Torvalds Style)
* **Sadelik ve Okunabilirlik:** Gereksiz Rust soyutlamalarından (ağır C++ şablonları benzeri jenerikler, karmaşık trait zincirleri) kaçının. Kod olabildiğince C mantığına yakın, düz ve okunabilir olmalıdır.
* **Donanım Dostu Entegrasyon:** Performans kritik işlemlerde (örn: bit tarama) Rust derleyicisinin doğrudan optimize donanım komutlarına (CLZ/bsr) dönüştüreceği native metodlar (Örn: `leading_zeros()`) tercih edilmelidir.
* **Sıfır Panik Riski (Panic-free):** Olası taşma ve mantık hatalarında çekirdeğin çökmesini/paniklemesini önlemek için sarmal matematik (`wrapping` metodları) kullanılmalıdır.

### 📌 Kural 2: Güvenli Kod Yazımı (Safe Code Only)
* **Bellek Güvenliği:** Kodların tamamı %100 güvenli (safe) Rust ile yazılmalıdır.
* **Unsafe Yasağı:** Çok ekstrem donanım veya doğrudan bellek erişimi gerektirmedikçe **hiçbir şekilde `unsafe` blokları kullanılmamalıdır**.

### 📌 Kural 3: C API ve ABI Uyumluluğunun Korunması
* **ABI Koruma:** Mevcut C API imzaları kesinlikle değiştirilmemelidir. Sürücülerin ve diğer C modüllerinin çağrı yapabilmesi için fonksiyonlar `#[no_mangle] pub extern "C"` niteliği ile tanımlanmalıdır.
* **Koşullu Derleme:** Makefile üzerinde `CONFIG_RUST` durumuna göre koşullu derleme yapılarak, çift sembol (duplicate symbol) linker çakışmaları önlenmelidir.

### 📌 Kural 4: Kod İçi Dil Standardı (English Comments & Docs)
* **İngilizce Kod Açıklamaları:** Çekirdeğin ana dil standardına uyum sağlamak amacıyla, kod dosyaları (`.rs`) içindeki tüm açıklama satırları (`//`) ve API dokümantasyonları (`///` / `//!`) **tamamen İngilizce** yazılmalıdır.

### 📌 Kural 5: Belgelendirme Zorunluluğu (Migration Logging)
* **Zorunlu Günlük Kaydı:** Yapılan her değişiklik, her taşınan fonksiyon ve karşılaşılan her kritik karar satır satır [migration_log.md](file:///home/aethelis/Runix/Knowledge/migration_log.md) dosyasına işlenmelidir.
