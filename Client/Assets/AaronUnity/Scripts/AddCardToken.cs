using System;
using System.Collections;
using System.Collections.Generic;
using TMPro;
using UnityEngine;
using UnityEngine.UI;

public static class ButtonExtension
{
    public static void addEventListener<T>(this Button button, T param, Action<T> OnClick)
    {
        button.onClick.AddListener(delegate ()
        {
            OnClick(param);
        });
    }
}
public class AddCardToken : MonoBehaviour
{
    public GameObject templateCard;
    GameObject g;
    [SerializeField] Transform CardsListUI;

    [SerializeField] GameObject imageHolder;
    [SerializeField] GameObject statsHolder;

    [SerializeField] GameObject OptionButtonsHolder;
    [SerializeField] GameObject UpgradeHeroesHolder;
    [SerializeField] GameObject OddsBurnHolder;

    [SerializeField] GameObject OddsBurnHolder_fieds;

    [SerializeField] GameObject btnCancel;

    [SerializeField] Sprite overrideImage;
    [SerializeField] TextMeshProUGUI TopTitle;

    private int i;
    private int selectedindex = 0;
    private string selectionState;


    private List<GameObject> selectedBoth;

    private CardDisplay tempGameObject;

    void Start()
    {
        i = 0;
        selectionState = "selection_hero";
        selectedBoth = new List<GameObject>();
    }
    void Update()
    {
        if (selectedBoth.Count == 1)
        {
            TopTitle.text = "Choose One More Hero To Burn";
        }

        else if (selectedBoth.Count == 2)
        {
            TopTitle.text = "Ready To Upgrade";
        }

    }
    public void addCardToken()
    {
        g = Instantiate(templateCard, CardsListUI);
        g.transform.GetChild(0).GetComponent<Button>().addEventListener(i, onClickEvent);
        InstantiatedTokens.instantiatedObjects.Add(g);
        i++;
    }

    void onClickEvent(int index)
    {
        printIndex(index);
        selectedindex = index;

        #region Check Selection State
        if (selectionState.Equals("selection_hero"))
        {
            imageHolder.SetActive(false);
            statsHolder.SetActive(true);
            btnCancel.SetActive(true);
            UpgradeHeroesHolder.SetActive(false);
            OptionButtonsHolder.SetActive(true);
        }
        else if (selectionState.Equals("selection_to_burn"))
        {
            selectedBoth.Add(g);

            InstantiatedTokens.instantiatedObjects[selectedindex].gameObject.transform.GetChild(0).GetComponent<Button>().interactable = false;

        }
        #endregion

        if (!selectionState.Equals("selection_hero")) return;

        tempGameObject = InstantiatedTokens.instantiatedObjects[index].gameObject.GetComponent<CardDisplay>();

        #region Update Thumbnail Stats
        statsHolder.transform.GetChild(1).GetChild(0).GetComponentInChildren<TextMeshProUGUI>().text = InstantiatedTokens.instantiatedObjects[index].gameObject.GetComponent<CardDisplay>().txtStat1.text;

        statsHolder.transform.GetChild(1).GetChild(1).GetComponentInChildren<TextMeshProUGUI>().text = InstantiatedTokens.instantiatedObjects[index].gameObject.GetComponent<CardDisplay>().txtStat2.text;

        statsHolder.transform.GetChild(1).GetChild(2).GetComponentInChildren<TextMeshProUGUI>().text = InstantiatedTokens.instantiatedObjects[index].gameObject.GetComponent<CardDisplay>().txtStat3.text;

        statsHolder.transform.GetChild(1).GetChild(3).GetComponentInChildren<TextMeshProUGUI>().text = InstantiatedTokens.instantiatedObjects[index].gameObject.GetComponent<CardDisplay>().txtStat4.text;

        statsHolder.transform.GetChild(1).GetChild(4).GetComponentInChildren<TextMeshProUGUI>().text = InstantiatedTokens.instantiatedObjects[index].gameObject.GetComponent<CardDisplay>().txtName.text;

        #endregion

    }

    public void onStartBothSelection()
    {
        if (InstantiatedTokens.instantiatedObjects.Count < 3) return;

        selectionState = "selection_to_burn";
        TopTitle.text = "Choose Two Heroes To Burn";

        OptionButtonsHolder.SetActive(false);
        UpgradeHeroesHolder.SetActive(true);

        InstantiatedTokens.instantiatedObjects[selectedindex].gameObject.transform.GetChild(0).GetComponent<Button>().image.overrideSprite = overrideImage;

        InstantiatedTokens.instantiatedObjects[selectedindex].gameObject.transform.GetChild(0).GetComponent<Button>().interactable = false;
    }

    public void onCloseOddBurnPanel()
    {
        OddsBurnHolder.SetActive(false);
    }

    public void onOpenOddBurnPanel()
    {
        OddsBurnHolder.SetActive(true);

        #region Fill Selected Main Card
        OddsBurnHolder_fieds.transform.GetChild(0).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat1.text;
        OddsBurnHolder_fieds.transform.GetChild(1).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat2.text;
        OddsBurnHolder_fieds.transform.GetChild(2).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat3.text;
        OddsBurnHolder_fieds.transform.GetChild(3).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat4.text;
        #endregion


        #region Fill Green 33% Burned Card
        OddsBurnHolder_fieds.transform.GetChild(4).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat1.text;
        OddsBurnHolder_fieds.transform.GetChild(5).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat2.text;
        OddsBurnHolder_fieds.transform.GetChild(6).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat3.text;
        OddsBurnHolder_fieds.transform.GetChild(7).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat4.text;
        #endregion

        #region Fill White 33% Burned Card
        OddsBurnHolder_fieds.transform.GetChild(8).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat1.text;
        OddsBurnHolder_fieds.transform.GetChild(9).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat2.text;
        OddsBurnHolder_fieds.transform.GetChild(10).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat3.text;
        OddsBurnHolder_fieds.transform.GetChild(11).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat4.text;
        #endregion

        #region Fill Red 33% Burned Card
        OddsBurnHolder_fieds.transform.GetChild(12).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat1.text;
        OddsBurnHolder_fieds.transform.GetChild(13).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat2.text;
        OddsBurnHolder_fieds.transform.GetChild(14).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat3.text;
        OddsBurnHolder_fieds.transform.GetChild(15).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat4.text;
        #endregion


        #region Fill Small Green 33% Burned Card
        OddsBurnHolder_fieds.transform.GetChild(16).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat1.text;
        OddsBurnHolder_fieds.transform.GetChild(17).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat2.text;
        OddsBurnHolder_fieds.transform.GetChild(18).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat3.text;
        OddsBurnHolder_fieds.transform.GetChild(19).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat4.text;
        #endregion

        #region Fill Small White 33% Burned Card
        OddsBurnHolder_fieds.transform.GetChild(20).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat1.text;
        OddsBurnHolder_fieds.transform.GetChild(21).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat2.text;
        OddsBurnHolder_fieds.transform.GetChild(22).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat3.text;
        OddsBurnHolder_fieds.transform.GetChild(23).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat4.text;
        #endregion

        #region Fill Small Red 33% Burned Card
        OddsBurnHolder_fieds.transform.GetChild(24).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat1.text;
        OddsBurnHolder_fieds.transform.GetChild(25).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat2.text;
        OddsBurnHolder_fieds.transform.GetChild(26).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat3.text;
        OddsBurnHolder_fieds.transform.GetChild(27).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat4.text;
        #endregion


        #region Fill Extra Small Green 33% Burned Card
        setColorOfFiled(OddsBurnHolder_fieds.transform.GetChild(28).GetComponent<TextMeshProUGUI>(), UnityEngine.Random.Range(0, 4));
        setColorOfFiled(OddsBurnHolder_fieds.transform.GetChild(29).GetComponent<TextMeshProUGUI>(), UnityEngine.Random.Range(0, 4));
        setColorOfFiled(OddsBurnHolder_fieds.transform.GetChild(30).GetComponent<TextMeshProUGUI>(), UnityEngine.Random.Range(0, 4));
        setColorOfFiled(OddsBurnHolder_fieds.transform.GetChild(31).GetComponent<TextMeshProUGUI>(), UnityEngine.Random.Range(0, 4));

        setSignOfFiled(OddsBurnHolder_fieds.transform.GetChild(28).GetComponent<TextMeshProUGUI>(), UnityEngine.Random.Range(0, 4));
        setSignOfFiled(OddsBurnHolder_fieds.transform.GetChild(29).GetComponent<TextMeshProUGUI>(), UnityEngine.Random.Range(0, 4));
        setSignOfFiled(OddsBurnHolder_fieds.transform.GetChild(30).GetComponent<TextMeshProUGUI>(), UnityEngine.Random.Range(0, 4));
        setSignOfFiled(OddsBurnHolder_fieds.transform.GetChild(31).GetComponent<TextMeshProUGUI>(), UnityEngine.Random.Range(0, 4));

        updateOldField(OddsBurnHolder_fieds.transform.GetChild(4).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(28).GetComponent<TextMeshProUGUI>().text));
        updateOldField(OddsBurnHolder_fieds.transform.GetChild(5).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(29).GetComponent<TextMeshProUGUI>().text));
        updateOldField(OddsBurnHolder_fieds.transform.GetChild(6).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(30).GetComponent<TextMeshProUGUI>().text)); updateOldField(OddsBurnHolder_fieds.transform.GetChild(7).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(31).GetComponent<TextMeshProUGUI>().text));

        setColorOfFiled(OddsBurnHolder_fieds.transform.GetChild(4).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(28).GetComponent<TextMeshProUGUI>().text));
        setColorOfFiled(OddsBurnHolder_fieds.transform.GetChild(5).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(29).GetComponent<TextMeshProUGUI>().text));
        setColorOfFiled(OddsBurnHolder_fieds.transform.GetChild(6).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(30).GetComponent<TextMeshProUGUI>().text));
        setColorOfFiled(OddsBurnHolder_fieds.transform.GetChild(7).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(31).GetComponent<TextMeshProUGUI>().text));

        #endregion

        #region Fill Extra Small White 33% Burned Card
        OddsBurnHolder_fieds.transform.GetChild(32).GetComponent<TextMeshProUGUI>().text = "+0";
        OddsBurnHolder_fieds.transform.GetChild(33).GetComponent<TextMeshProUGUI>().text = "+0";
        OddsBurnHolder_fieds.transform.GetChild(34).GetComponent<TextMeshProUGUI>().text = "+0";
        OddsBurnHolder_fieds.transform.GetChild(35).GetComponent<TextMeshProUGUI>().text = "+0";
        #endregion

        #region Fill Extra Small Red 33% Burned Card

        setColorOfFiled(OddsBurnHolder_fieds.transform.GetChild(36).GetComponent<TextMeshProUGUI>(), UnityEngine.Random.Range(0, -4));
        setColorOfFiled(OddsBurnHolder_fieds.transform.GetChild(37).GetComponent<TextMeshProUGUI>(), UnityEngine.Random.Range(0, -4));
        setColorOfFiled(OddsBurnHolder_fieds.transform.GetChild(38).GetComponent<TextMeshProUGUI>(), UnityEngine.Random.Range(0, -4));
        setColorOfFiled(OddsBurnHolder_fieds.transform.GetChild(39).GetComponent<TextMeshProUGUI>(), UnityEngine.Random.Range(0, -4));

        setSignOfFiled(OddsBurnHolder_fieds.transform.GetChild(36).GetComponent<TextMeshProUGUI>(), UnityEngine.Random.Range(0, -4));
        setSignOfFiled(OddsBurnHolder_fieds.transform.GetChild(37).GetComponent<TextMeshProUGUI>(), UnityEngine.Random.Range(0, -4));
        setSignOfFiled(OddsBurnHolder_fieds.transform.GetChild(38).GetComponent<TextMeshProUGUI>(), UnityEngine.Random.Range(0, -4));
        setSignOfFiled(OddsBurnHolder_fieds.transform.GetChild(39).GetComponent<TextMeshProUGUI>(), UnityEngine.Random.Range(0, -4));

        updateOldField(OddsBurnHolder_fieds.transform.GetChild(12).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(36).GetComponent<TextMeshProUGUI>().text));
        updateOldField(OddsBurnHolder_fieds.transform.GetChild(13).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(37).GetComponent<TextMeshProUGUI>().text));
        updateOldField(OddsBurnHolder_fieds.transform.GetChild(14).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(38).GetComponent<TextMeshProUGUI>().text)); updateOldField(OddsBurnHolder_fieds.transform.GetChild(15).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(39).GetComponent<TextMeshProUGUI>().text));

        setColorOfFiled(OddsBurnHolder_fieds.transform.GetChild(12).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(36).GetComponent<TextMeshProUGUI>().text));
        setColorOfFiled(OddsBurnHolder_fieds.transform.GetChild(13).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(37).GetComponent<TextMeshProUGUI>().text));
        setColorOfFiled(OddsBurnHolder_fieds.transform.GetChild(14).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(38).GetComponent<TextMeshProUGUI>().text));
        setColorOfFiled(OddsBurnHolder_fieds.transform.GetChild(15).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(39).GetComponent<TextMeshProUGUI>().text));


        #endregion


        #region Fill Card Name
        OddsBurnHolder_fieds.transform.GetChild(40).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtName.text;
        #endregion

    }

    private void printIndex(int index)
    {
        Debug.Log("selected index: " + index);
    }


    private void setColorOfFiled(TextMeshProUGUI entity, int value)
    {
        if (value > 0) entity.color = new Color(0, 1, 0);
        else if (value < 0) entity.color = new Color(1, 0, 0);
        else entity.color = new Color(1, 1, 1);
    }

    private void setSignOfFiled(TextMeshProUGUI entity, int value)
    {
        if (value > 0) entity.text = "+" + value;
        else if (value == 0) entity.text = "+" + value;
        else entity.text = "" + value;
    }

    private void updateOldField(TextMeshProUGUI entity, int value)
    {
        entity.text = (Convert.ToInt16(entity.text) + value).ToString();
    }



}
