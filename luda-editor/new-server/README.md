# DB Strategy

- This system uses sqlite as the main database, stored in AWS EBS, periodically backup to S3.
- Data could be lost if EBS failed before backing up to S3.
- Use SQS or other method to save data safely, for example, purchase.
