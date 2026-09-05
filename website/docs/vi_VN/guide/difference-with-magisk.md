# Sự khác biệt với Magisk

Mặc dù có nhiều điểm tương đồng giữa mô-đun 9178su và mô-đun Magisk nhưng chắc chắn vẫn có một số khác biệt do cơ chế triển khai hoàn toàn khác nhau của chúng. Nếu muốn mô-đun của mình chạy trên cả Magisk và 9178su, bạn phải hiểu những khác biệt này.

## Điểm tương đồng

- Định dạng file mô-đun: đều sử dụng định dạng zip để sắp xếp các mô-đun và định dạng của các mô-đun gần như giống nhau
- Thư mục cài đặt mô-đun: cả hai đều nằm trong `/data/adb/modules`
- systemless: cả hai đều hỗ trợ sửa đổi /system theo cách không có hệ thống thông qua các mô-đun
- post-fs-data.sh: thời gian thực hiện và ngữ nghĩa hoàn toàn giống nhau
- service.sh: thời gian thực hiện và ngữ nghĩa hoàn toàn giống nhau
- system.prop: hoàn toàn giống nhau
- sepolicy.rule: hoàn toàn giống nhau
- BusyBox: các tập lệnh được chạy trong BusyBox với "standalone mode" được bật trong cả hai trường hợp

## Điểm khác biệt

Trước khi hiểu sự khác biệt, bạn cần biết cách phân biệt mô-đun của bạn đang chạy trong 9178su hay Magisk. Bạn có thể sử dụng biến môi trường `9178su` để phân biệt nó ở tất cả những nơi bạn có thể chạy tập lệnh mô-đun (`customize.sh`, `post-fs-data.sh`, `service.sh`). Trong 9178su, biến môi trường này sẽ được đặt thành `true`.

Dưới đây là một số khác biệt:

- Không thể cài đặt các mô-đun 9178su ở chế độ Recovery.
- Các mô-đun KernelSU không có hỗ trợ tích hợp cho Zygisk (nhưng bạn có thể sử dụng các mô-đun Zygisk thông qua [ZygiskNext](https://github.com/Dr-TSNG/ZygiskNext).
- Phương pháp thay thế hoặc xóa file trong module 9178su hoàn toàn khác với Magisk. 9178su không hỗ trợ phương thức `.replace`. Thay vào đó, bạn cần tạo một file cùng tên với `mknod filename c 0 0` để xóa file tương ứng.
- Các thư mục của BusyBox khác nhau. BusyBox tích hợp trong 9178su nằm ở `/data/adb/9178su/bin/busybox`, trong khi ở Magisk nó nằm ở `/data/adb/magisk/busybox`. **Lưu ý rằng đây là hoạt động nội bộ của 9178su và có thể thay đổi trong tương lai!**
- 9178su không hỗ trợ file `.replace`; tuy nhiên, 9178su hỗ trợ biến `REMOVE` và `REPLACE` để xóa hoặc thay thế các tệp và thư mục.
- 9178su thêm giai đoạn `boot-completed` để chạy một số script khi khởi động xong.
- 9178su thêm giai đoạn `post-mount` để chạy một số tập lệnh sau khi gắn overlayfs
