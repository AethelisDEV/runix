# Linux Çekirdeği C -> Rust Taşıma Günlüğü (Migration Log)

Bu günlük, Linux çekirdeğinin C tabanlı yardımcı fonksiyonlarını ve kütüphanelerini aşamalı olarak Rust diline taşırken gerçekleştirdiğimiz tüm kurulum adımlarını, mimari kararları ve dosya bazındaki değişiklikleri satır satır belgelemektedir.

---

## 1. Ön Hazırlıklar ve Kurulum Adımları

Çekirdek derleme ve Rust entegrasyonu öncesinde yapılan sistem düzeyindeki hazırlıklar:

* **Çalışma Alanı Dizini Düzeltmesi:**
  * **Sorun:** Linux derleme sistemi (Kbuild), boşluk veya `:` karakteri içeren yollarda derleme yapılmasına izin vermez. Proje ilk olarak `/run/media/aethelis/Gen4 M2/AE Projects/Runix` yolundaydı.
  * **Çözüm:** Proje boşluk içermeyen `/home/aethelis/Runix` yoluna taşındı.
* **Rust Standart Kütüphane Kaynak Kodu:**
  * Çekirdeğin `core` kütüphanesini çapraz derleyebilmesi için Rust kaynak kodları yüklendi:
    ```bash
    rustup component add rust-src
    ```
* **Bindgen Kurulumu:**
  * C başlık dosyalarından otomatik Rust bağlayıcı kodları üreten `bindgen` kuruldu:
    ```bash
    cargo install --locked bindgen-cli
    ```
* **Derleme Bağımlılığı (`bc`):**
  * Çekirdeğin zamanlayıcı sabitlerini hazırlık aşamasında hesaplayan `bc` aracı sisteme kuruldu (`sudo pacman -S bc`).
* **Çekirdek Yapılandırması (`.config`):**
  * Rust ve birim testleri (KUnit) etkinleştirildi:
    ```bash
    CONFIG_RUST=y
    CONFIG_KUNIT=y
    CONFIG_GCD_KUNIT_TEST=y
    ```
  * Değişikliklerin uygulanması için `make olddefconfig` çalıştırıldı.

---

## 2. Taşınan Fonksiyonlar ve Dosya Değişiklikleri

### A. GCD (En Büyük Ortak Bölen) Fonksiyonu

* **Kaynak C Dosyası:** [lib/math/gcd.c](file:///home/aethelis/Runix/lib/math/gcd.c)
* **İlişkili C Başlık Dosyası:** [include/linux/gcd.h](file:///home/aethelis/Runix/include/linux/gcd.h) (Değiştirilmedi)
* **Rust Modülü Kaydı:**
  * [rust/kernel/lib.rs](file:///home/aethelis/Runix/rust/kernel/lib.rs) dosyasına `pub mod math;` eklendi.
* **Yazılan Rust Kodu:**
  * [rust/kernel/math.rs](file:///home/aethelis/Runix/rust/kernel/math.rs) dosyası oluşturuldu ve Stein ikili EBOB algoritması güvenli (safe) Rust ile yazıldı:
    ```rust
    #[no_mangle]
    pub extern "C" fn gcd(mut a: c_ulong, mut b: c_ulong) -> c_ulong { ... }
    ```
* **Makefile Koşullandırması:**
  * [lib/math/Makefile](file:///home/aethelis/Runix/lib/math/Makefile) içinde `gcd.o` koşullu hale getirildi:
    ```makefile
    ifneq ($(CONFIG_RUST),y)
    obj-y += gcd.o
    endif
    ```

---

### B. LCM (En Küçük Ortak Kat) Fonksiyonları

* **Kaynak C Dosyası:** [lib/math/lcm.c](file:///home/aethelis/Runix/lib/math/lcm.c)
* **İlişkili C Başlık Dosyası:** [include/linux/lcm.h](file:///home/aethelis/Runix/include/linux/lcm.h) (Değiştirilmedi)
* **Yazılan Rust Kodu:**
  * [rust/kernel/math.rs](file:///home/aethelis/Runix/rust/kernel/math.rs) dosyasına `lcm` ve `lcm_not_zero` fonksiyonları eklendi.
  * **Kritik Karar:** Rust'ın taşma (overflow) kontrollerine takılıp panik üretmesini önlemek için C ile birebir aynı davranan sarmal aritmetik (`wrapping_div` ve `wrapping_mul`) kullanıldı:
    ```rust
    #[no_mangle]
    pub extern "C" fn lcm(a: c_ulong, b: c_ulong) -> c_ulong {
        if a != 0 && b != 0 {
            a.wrapping_div(gcd(a, b)).wrapping_mul(b)
        } else {
            0
        }
    }
    ```
* **Makefile Koşullandırması:**
  * [lib/math/Makefile](file:///home/aethelis/Runix/lib/math/Makefile) içinde `lcm.o` koşullu hale getirildi:
    ```makefile
    ifneq ($(CONFIG_RUST),y)
    obj-y += gcd.o lcm.o
    endif
    ```

---

## 3. Doğrulama ve Test Sonuçları

* **İhraç Edilen Semboller:**
  * Derleme sonrasında `rust/exports_kernel_generated.h` dosyasında aşağıdaki sembollerin GPL uyumlu olarak ihraç edildiği doğrulandı:
    ```c
    EXPORT_SYMBOL_RUST_GPL(gcd);
    EXPORT_SYMBOL_RUST_GPL(lcm);
    EXPORT_SYMBOL_RUST_GPL(lcm_not_zero);
    ```
* **KUnit Testleri:**
  * UML (User Mode Linux) üzerinde `.kunitconfig` ile tüm testler koşturuldu:
    ```bash
    ./tools/testing/kunit/kunit.py run --make_options LLVM=1 --kunitconfig=.kunitconfig
    ```
  * `math-gcd` birim testlerindeki 11 test senaryosunun tamamının başarıyla geçtiği doğrulandı.

---

### C. int_pow (Tam Sayı Üs Alma) Fonksiyonu

* **Kaynak C Dosyası:** [lib/math/int_pow.c](file:///home/aethelis/Runix/lib/math/int_pow.c)
* **İlişkili C Başlık Dosyası:** [include/linux/math.h](file:///home/aethelis/Runix/include/linux/math.h) (Değiştirilmedi)
* **Yazılan Rust Kodu:**
  * [rust/kernel/math.rs](file:///home/aethelis/Runix/rust/kernel/math.rs) dosyasına `int_pow` fonksiyonu eklendi.
  * **Kritik Karar:** Rust'ın taşma (overflow) kontrollerine takılıp panik üretmesini önlemek ve C ile aynı sarmal çarpan yapısını sürdürmek için `wrapping_mul` sarmal aritmetiği kullanıldı:
    ```rust
    #[no_mangle]
    pub extern "C" fn int_pow(mut base: u64, mut exp: core::ffi::c_uint) -> u64 {
        let mut result: u64 = 1;
        while exp != 0 {
            if (exp & 1) != 0 {
                result = result.wrapping_mul(base);
            }
            exp >>= 1;
            base = base.wrapping_mul(base);
        }
        result
    }
    ```
* **Makefile Koşullandırması:**
  * [lib/math/Makefile](file:///home/aethelis/Runix/lib/math/Makefile) içinde `int_pow.o` koşullu hale getirildi:
    ```makefile
    ifneq ($(CONFIG_RUST),y)
    obj-y += gcd.o lcm.o int_pow.o
    endif
    ```
* **KUnit Test Doğrulamaları:**
  * `.kunitconfig` dosyasına `CONFIG_INT_POW_KUNIT_TEST=y` eklenerek testler UML üzerinde koşturuldu.
  * `math-int_pow` birim testlerindeki 9 test senaryosunun tamamının başarıyla geçtiği doğrulandı.

---

### D. int_sqrt (Tam Sayı Karekök) Fonksiyonları

* **Kaynak C Dosyası:** [lib/math/int_sqrt.c](file:///home/aethelis/Runix/lib/math/int_sqrt.c)
* **İlişkili C Başlık Dosyası:** [include/linux/math.h](file:///home/aethelis/Runix/include/linux/math.h) (Değiştirilmedi)
* **Yazılan Rust Kodu:**
  * [rust/kernel/math.rs](file:///home/aethelis/Runix/rust/kernel/math.rs) dosyasına `int_sqrt` ve 32-bit mimariler için `int_sqrt64` fonksiyonları eklendi.
  * **Linus Torvalds Tasarımı:** 
    * `__fls` (en yüksek anlamlı biti bulma) donanım destekli assembly komutlarına (`bsr`/`clz`) derlenen Rust'ın native `leading_zeros()` metoduyla performanslı bir şekilde yazıldı:
      `let fls = (usize::BITS - 1).wrapping_sub(x.leading_zeros() as u32);`
    * Algoritma herhangi bir `unsafe` blok içermeyecek şekilde sıfır hata ve panik riskiyle geliştirildi.
    * 32-bit platformlardaki 64-bit girdileri destekleyen `int_sqrt64` fonksiyonu `#[cfg(target_pointer_width = "32")]` ile koşullandırıldı. Bu sayede 64-bit platformlardaki `static inline` C tanımlamalarıyla çakışması önlendi.
* **Makefile Koşullandırması:**
  * [lib/math/Makefile](file:///home/aethelis/Runix/lib/math/Makefile) içinde `int_sqrt.o` koşullu hale getirildi:
    ```makefile
    ifneq ($(CONFIG_RUST),y)
    obj-y += gcd.o lcm.o int_pow.o int_sqrt.o
    endif
    ```
* **KUnit Test Doğrulamaları:**
  * `.kunitconfig` dosyasına `CONFIG_INT_SQRT_KUNIT_TEST=y` eklenerek testler UML üzerinde koşturuldu.
  * `math-int_sqrt` birim test grubundaki 21 karekök test senaryosunun (0, 1, tam kare olmayanlar, maksimum değer sınırları vb.) tamamının başarıyla geçtiği doğrulandı.

---

### E. reciprocal_div (Karşılıklı Bölme) Fonksiyonları

* **Kaynak C Dosyası:** [lib/math/reciprocal_div.c](file:///home/aethelis/Runix/lib/math/reciprocal_div.c)
* **İlişkili C Başlık Dosyası:** [include/linux/reciprocal_div.h](file:///home/aethelis/Runix/include/linux/reciprocal_div.h) (Değiştirilmedi)
* **Veri Yapılarının Rust Tarafına Eklenmesi:**
  * [rust/bindings/bindings_helper.h](file:///home/aethelis/Runix/rust/bindings/bindings_helper.h) dosyasına `#include <linux/reciprocal_div.h>` eklendi. Böylece `struct reciprocal_value` ve `struct reciprocal_value_adv` yapılarının Rust bindings katmanında FFI uyumlu olarak otomatik üretilmesi sağlandı.
* **Yazılan Rust Kodu:**
  * [rust/kernel/math.rs](file:///home/aethelis/Runix/rust/kernel/math.rs) dosyasına `reciprocal_value` ve `reciprocal_value_adv` fonksiyonları eklendi.
  * **Linus Torvalds Tasarımı:**
    * C'deki `do_div(m, d)` makrosunun (64-bit sayıyı 32-bit sayıya bölme) emülasyonu yerine, Rust'ın otomatik derleyici desteği sunan yerleşik `/` bölme operatörü kullanıldı.
    * Herhangi bir bellek güvenliği riski taşımayan sıfır `unsafe` içeren kod tasarımı korundu.
    * Hataları önlemek adına sarmallama (`wrapping_mul`, `wrapping_sub`, `wrapping_div`, `wrapping_shl`) aritmetikleriyle güvenli sınır hesaplamaları yapıldı.
* **Makefile Koşullandırması:**
  * [lib/math/Makefile](file:///home/aethelis/Runix/lib/math/Makefile) içinde `reciprocal_div.o` koşullu hale getirildi:
    ```makefile
    ifneq ($(CONFIG_RUST),y)
    obj-y += gcd.o lcm.o int_pow.o int_sqrt.o reciprocal_div.o
    endif
    ```
* **Doğrulama Sonuçları:**
  * UML üzerinde KUnit testleri koşturularak derleme hataları ve linker çakışmaları olmadığı doğrulandı.
  * Sembol tablosunda (`exports_kernel_generated.h`) `reciprocal_value` ve `reciprocal_value_adv` fonksiyonlarının düzgün ihraç edildiği onaylandı.

---

### F. rational (Rasyonel Sayı Yakınsama) Fonksiyonu

* **Kaynak C Dosyası:** [lib/math/rational.c](file:///home/aethelis/Runix/lib/math/rational.c)
* **İlişkili C Başlık Dosyası:** [include/linux/rational.h](file:///home/aethelis/Runix/include/linux/rational.h) (Değiştirilmedi)
* **Yazılan Rust Kodu:**
  * [rust/kernel/math.rs](file:///home/aethelis/Runix/rust/kernel/math.rs) dosyasına `rational_best_approximation` fonksiyonu eklendi.
* **Linus Torvalds Tasarımı / Kural Entegrasyonu:**
    * **FFI Gösterici Yazımı (Kural 2):** Sonuçları C çağırıcısına dönmek için pointer yazma işlemi (`*mut c_ulong`) zorunluydu. Pointer dereferans işlemleri öncesinde null kontrolleri yapıldı ve `unsafe` bloğu sadece gerçek yazım anı ile sınırlandırıldı.
    * **İngilizce Kod Açıklamaları (Kural 4):** Dosya içindeki tüm yorumlar ve docstring açıklamaları İngilizce olarak kaleme alındı.
    * **Sarmallama Aritmetiği:** Olası taşma paniklerini engellemek için sarmal matematik (`wrapping_add`, `wrapping_mul`, `wrapping_sub`, `wrapping_div`) kullanıldı.
* **Makefile ve Kconfig Koşullandırması:**
  * `lib/math/Kconfig` dosyasındaki hidden `config RATIONAL` seçeneğine başlık (prompt) verilerek elle seçilebilir hale getirildi ve `.kunitconfig` dosyasına `CONFIG_RATIONAL=y` ve `CONFIG_RATIONAL_KUNIT_TEST=y` eklenerek KUnit testleri aktif edildi.
  * [lib/math/Makefile](file:///home/aethelis/Runix/lib/math/Makefile) içinde `rational.o` derlemesi, `CONFIG_RUST` aktif olduğunda hem `obj-y` hem de `obj-$(CONFIG_RATIONAL)` kısımlarında derlenmeyecek şekilde koşullandırıldı:
    ```makefile
    ifneq ($(CONFIG_RUST),y)
    obj-y += gcd.o lcm.o int_pow.o int_sqrt.o reciprocal_div.o
    obj-$(CONFIG_RATIONAL)		+= rational.o
    endif
    ```
* **KUnit Test Doğrulamaları:**
  * UML üzerinde `math-rational` birim testlerindeki 8 yakınsama senaryosunun tamamının başarıyla geçtiği doğrulandı.

---

### G. int_log (Sabit Noktalı Logaritma) Fonksiyonları ve Matematik Kütüphanesi Modüler Yapılandırması

* **Kaynak C Dosyası:** [lib/math/int_log.c](file:///home/aethelis/Runix/lib/math/int_log.c)
* **İlişkili C Başlık Dosyası:** [include/linux/int_log.h](file:///home/aethelis/Runix/include/linux/int_log.h) (Değiştirilmedi)
* **Matematik Modülü Klasör Yapılandırması (Refactoring):**
  * **Mevcut Sorun:** `rust/kernel/math.rs` dosyası tüm fonksiyonlar tek bir yerde toplandığı için büyümüştü. Çekirdeğin genel Rust yapısına uygun olarak alt modüllere bölünmesi kararlaştırıldı.
  * **Yeni Klasör Yapısı:** `rust/kernel/math/` dizini oluşturuldu ve fonksiyonlar şu alt dosyalara ayrıldı:
    * `gcd.rs`: binary GCD algoritması.
    * `lcm.rs`: LCM ve LCM not zero fonksiyonları.
    * `int_pow.rs`: tam sayı üs alma.
    * `int_sqrt.rs`: tam sayı karekök.
    * `reciprocal.rs`: karşılıklı bölme sabitleri.
    * `rational.rs`: rasyonel sayı yakınsama.
    * `int_log.rs`: sabit noktalı logaritma (yeni port edilen modül).
  * `rust/kernel/math.rs` dosyası alt modülleri tanımlayıp sembolleri `pub use` ile yönlendirecek şekilde güncellendi.
* **Yazılan Rust Kodu:**
  * [rust/kernel/math/int_log.rs](file:///home/aethelis/Runix/rust/kernel/math/int_log.rs) dosyası oluşturuldu.
  * `logtable` dizisi `const LOGTABLE: [u16; 256]` olarak Rust tarafına aktarıldı.
  * `intlog2` ve `intlog10` fonksiyonları tamamen safe Rust ile yazıldı.
  * **Warn_on Makrosu Entegrasyonu:** `value == 0` durumlarında warning üretmek için çekirdeğin `crate::warn_on!` makrosu kullanıldı. Bu makronun macro expansion aşamasında hata üretmemesi için `use crate::str::CStrExt;` trait'i modüle dahil edildi.
  * **Sarmal Aritmetik:** Çarpma, çıkarma ve bit kaydırma işlemlerinde integer overflow önleyici `wrapping_*` metodları kullanıldı.
* **Makefile ve Kconfig Koşullandırması:**
  * [lib/math/Makefile](file:///home/aethelis/Runix/lib/math/Makefile) güncellenerek `CONFIG_RUST` aktif olduğunda `int_log.o` C derlemesi devredışı bırakıldı.
  * `.kunitconfig` dosyasına `CONFIG_INT_LOG_KUNIT_TEST=y` eklendi.
* **KUnit Test Doğrulamaları:**
  * UML üzerinde KUnit testleri koşturuldu:
    * `intlog2_test` içerisindeki 9 test senaryosu ve `intlog10_test` içerisindeki 8 test senaryosu (toplam 17 senaryo) başarıyla geçti.
    * Toplam test sayısı 405'e ulaştı ve tüm testler sorunsuz tamamlandı.

---

### H. CORDIC (Açı ve Koordinat Dönüşüm) Fonksiyonu

* **Kaynak C Dosyası:** [lib/math/cordic.c](file:///home/aethelis/Runix/lib/math/cordic.c)
* **İlişkili C Başlık Dosyası:** [include/linux/cordic.h](file:///home/aethelis/Runix/include/linux/cordic.h) (Değiştirilmedi)
* **FFI Entegrasyonu:**
  * `struct cordic_iq` yapısının ve CORDIC sabitlerinin Rust tarafına aktarılması amacıyla [rust/bindings/bindings_helper.h](file:///home/aethelis/Runix/rust/bindings/bindings_helper.h) dosyasına `#include <linux/cordic.h>` satırı eklendi.
* **Yazılan Rust Kodu:**
  * [rust/kernel/math/cordic.rs](file:///home/aethelis/Runix/rust/kernel/math/cordic.rs) dosyası oluşturularak CORDIC algoritması güvenli (safe) Rust ile yazıldı.
  * CORDIC sabitleri (`CORDIC_ANGLE_GEN`, vb.) ve `arctan_table` dizisi (`const ARCTAN_TABLE: [i32; 18]`) Rust'a aktarıldı.
  * `cordic_calc_iq` fonksiyonu `#[no_mangle] pub extern "C"` niteliği ile C ABI ve sembol uyumluluğu korunacak şekilde tanımlandı.
  * Belirsiz veri tiplerine karşı yerel değişkenlerin tipleri (`coord_i: i32`, `coord_q: i32`, vb.) açıkça tanımlandı ve taşma koruması amacıyla sarmal aritmetikler (`wrapping_shr`, `wrapping_sub`, vb.) kullanıldı.
* **Makefile ve Kconfig Koşullandırması:**
  * [lib/math/Makefile](file:///home/aethelis/Runix/lib/math/Makefile) dosyasında `obj-$(CONFIG_CORDIC) += cordic.o` derlemesi, `CONFIG_RUST` aktif olduğunda hem `obj-y` hem de `obj-$(CONFIG_CORDIC)` kısımlarında derlenmeyecek şekilde `ifneq ($(CONFIG_RUST),y)` bloğuna taşındı.
  * `.kunitconfig` dosyasına `CONFIG_CORDIC=y` seçeneği eklendi.
* **Doğrulama Sonuçları:**
  * Fonksiyonun doğruluğunu KUnit seviyesinde test etmek için `cordic.rs` dosyasına Rust doctest (`assert!((res.i - 65536).abs() <= 10)`) eklendi.
  * UML üzerinde KUnit testleri koşturularak Rust doctest'inin (`rust_doctest_kernel_math_cordic_rs_0`) başarıyla geçtiği ve tüm 406 testin tamamlandığı doğrulandı.


