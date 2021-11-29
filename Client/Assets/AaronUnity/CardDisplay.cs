using System.Collections;
using System.Collections.Generic;
using System.IO;
using TMPro;
using UnityEngine;
using UnityEngine.UI;

public class CardDisplay : MonoBehaviour
{
    public TextMeshProUGUI txtName;
    public TextMeshProUGUI txtStat1;
    public TextMeshProUGUI txtStat2;
    public TextMeshProUGUI txtStat3;
    public TextMeshProUGUI txtStat4;
<<<<<<< Updated upstream:Client/Assets/AaronUnity/CardDisplay.cs
    public GameObject InactifCardBurn;
    public bool isSelected;
=======
    public TextMeshProUGUI txtId;

    public Image image;
>>>>>>> Stashed changes:Client/Assets/CardDisplay.cs

    public CardToken cardToken;
    // Start is called before the first frame update
    void Start()
    {
        transform.localScale = new Vector3(0, 0, 0);
    }

    public void updateTokenValues(string n, int a, int b, int c, int d, string e, string sprite)
    {
        txtName.text = n;

        txtStat1.text = a.ToString();
        txtStat2.text = b.ToString();
        txtStat3.text = c.ToString();
        txtStat4.text = d.ToString();
        txtId.text = e.ToString();

        LoadImageFromDisk(sprite);


        LeanTween.scale(gameObject, new Vector3(1.65f, 1.65f, 1.65f), 0.2f).setEase(LeanTweenType.easeInQuad).setDelay(0.15f);
    }

    private void LoadImageFromDisk(string fileName)
    {
        byte[] textureBytes = File.ReadAllBytes(fileName);
        Texture2D loadedTexture = new Texture2D(0, 0);
        loadedTexture.LoadImage(textureBytes);
        image.sprite = Sprite.Create(loadedTexture, new Rect(0f, 0f, loadedTexture.width, loadedTexture.height), Vector2.zero);
    }

}