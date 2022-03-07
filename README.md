# تجربة بعض خصائص مكتبات التطوير NEAR Rust SDk Exploration

## لتشغيل المشروع

يحتاج المشروع لتثبيت أدوات التطوير الخاصة بلغة رست و ويب أسمبلي rust and web assembly tool chain
يمكن اتباع خطوات تثبيتهم هنــاhttps://www.near-sdk.io/

1- قم بنسخ المشروع لجهازك clone

2- من داخل مجلد المشروع فى شاشة الأوامر terminal قم بكتابة الأمر التالي

```
env 'RUSTFLAGS=-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
```

- ملحوظة: الجزء `env 'RUSTFLAGS=-C link-arg=-s'` يقوم بعمل تحسين لعملية تحويل الكود و يؤثر جذرياً على حجم الملف الناتج و بالتالى تكلفة رفع الملف و استضافته على البلوك تشين. إذا كان الملف الناتج من العملية السابقة أكبر من 200 كيلو بايت لديك (14 ميجا بايت مثلاً أو أكثر) فهذا يعنى أن الأمر لم يعمل بشكل صحيح و يجب عليك فحص إعدادات رست و تفعيله.

3- ستلاحظ وجود ملفات جديدة تم انشائها داخل مجلد target و منها الملف `near_rust_helloworld.wasm`

4- يمكنك رفع هذا الملف مباشرة على البلوك تشين بالطرق المعتادة مثل

```
 near dev-deploy .\target\wasm32-unknown-unknown\release\near-rust-sdk-exploration.wasm
```

5- يمكنك التجربة باستدعاء الدوال 

```
near view CONTRACT_ACCOUNT_ID get_message
```

```
near call CONTRACT_ACCOUNT_ID set_message '{"msg":"My Message"}' --accountId CALLER_ACCOUNT_ID
```

```
near view CONTRACT_ACCOUNT_ID get_message
```

```
near call CONTRACT_ACCOUNT_ID get_balance --accountId CALLER_ACCOUNT_ID
```
