#ifndef GPGMANAGER_H
#define GPGMANAGER_H

class GpgKeyComplete;

/*
GpgManager

Returns a unit through which GnuPG may be managed.
One can:
* Retrieve keys
* Sign
* Verify
* Encrypt files
* Decrypt files
* Read an encrypted file, then automatically delete
  the source
directly from a traditional C/C++ program, assuming
that the system has gpg installed.
*/
class GpgManager
{
public:
	GpgManager();
	virtual ~GpgManager();

	GpgKeyComplete** getKeys() const;
	GpgKeyComplete* getSecretKeys() const;
	int keyCount() const;

	bool sign(const char* filePath, bool clearSign = 0) const;
	bool verify(const char* key) const;

	// FILE* decryptThenRead(const char* inPath) const;
	// FILE* decrypt(const char* inPath, const char* outPath) const;
	// FILE* encrypt(const char* inPath, const char* outPath, const char* recipientID, bool armored = 0) const;

private:
	GpgKeyComplete** m_db;
	int m_size;	// Everything is padded out to 0b000 so this could even be a double.
};

#endif