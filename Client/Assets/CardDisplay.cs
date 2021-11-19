using System.Collections;
using System.Collections.Generic;
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

    public CardToken cardToken;
    // Start is called before the first frame update
    void Start()
    {
        txtName.text = cardToken.cardName;

        txtStat1.text = cardToken.cardWeapons.ToString();
        txtStat2.text = cardToken.cardEngineering.ToString();
        txtStat3.text = cardToken.cardBiotech.ToString();
        txtStat4.text = cardToken.cardPsychics.ToString();
        updateTokenValues();

    }

    private void updateTokenValues()
    {

        txtName.text = cardToken.cardName;

        txtStat1.text = Random.Range(0, 101).ToString();
        txtStat2.text = Random.Range(0, 101).ToString();
        txtStat3.text = Random.Range(0, 101).ToString();
        txtStat4.text = Random.Range(0, 101).ToString();

        transform.localScale = new Vector3(0, 0, 0);
        LeanTween.scale(gameObject, new Vector3(1.65f, 1.65f, 1.65f), 0.1f).setDelay(0.1f);

    }
}
